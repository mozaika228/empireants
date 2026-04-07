use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use empireants::observability::{encode_prometheus_with_metadata, ScrapeMetadata};
use empireants::simulation::{seeded_scale_world, AcoStrategy, Simulation, SimulationConfig};

const DEFAULT_BIND: &str = "127.0.0.1:9109";

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let bind = args
        .first()
        .cloned()
        .unwrap_or_else(|| DEFAULT_BIND.to_string());
    let ants = args
        .get(1)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(100_000);
    let width = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 2)
        .unwrap_or(384);
    let height = args
        .get(3)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 2)
        .unwrap_or(384);

    let mut simulation = Simulation::new(
        SimulationConfig {
            width,
            height,
            ant_count: ants,
            evaporation_rate: 0.04,
            diffusion_rate: 0.16,
            food_deposit: 0.7,
            home_deposit: 0.5,
            harvest_amount: 1,
            aco_strategy: AcoStrategy::MaxMin,
        },
        seeded_scale_world(width, height),
    );

    let latest = Arc::new(RwLock::new(String::from(
        "# empireants observability warming up\n",
    )));
    let scrape_count = Arc::new(AtomicU64::new(0));
    let started = Instant::now();
    spawn_http_server(
        &bind,
        Arc::clone(&latest),
        Arc::clone(&scrape_count),
        started,
    );
    println!(
        "observability server listening on http://{} (metrics: /metrics, health: /healthz)",
        bind
    );

    loop {
        simulation.step();
        if simulation.metrics().steps % 5 == 0 {
            let encoded = encode_prometheus_with_metadata(
                simulation.metrics(),
                simulation.runtime_snapshot(),
                ScrapeMetadata {
                    uptime_seconds: started.elapsed().as_secs_f64(),
                    scrape_count: scrape_count.load(Ordering::Relaxed),
                },
            );
            if let Ok(mut guard) = latest.write() {
                *guard = encoded;
            }
        }
    }
}

fn spawn_http_server(
    bind: &str,
    latest_metrics: Arc<RwLock<String>>,
    scrape_count: Arc<AtomicU64>,
    started: Instant,
) {
    let bind = bind.to_string();
    thread::spawn(move || {
        let listener = match TcpListener::bind(&bind) {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("failed to bind observability server at {bind}: {error}");
                std::process::exit(2);
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    if let Err(error) =
                        handle_client(&mut stream, &latest_metrics, &scrape_count, started)
                    {
                        eprintln!("observability server client error: {error}");
                    }
                }
                Err(error) => {
                    eprintln!("observability server accept error: {error}");
                    thread::sleep(Duration::from_millis(25));
                }
            }
        }
    });
}

fn handle_client(
    stream: &mut TcpStream,
    latest_metrics: &Arc<RwLock<String>>,
    scrape_count: &Arc<AtomicU64>,
    started: Instant,
) -> std::io::Result<()> {
    let mut buffer = [0_u8; 2048];
    let read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let first_line = request.lines().next().unwrap_or_default();

    if first_line.starts_with("GET /healthz") {
        write_response(stream, 200, "text/plain; charset=utf-8", "ok\n")?;
        return Ok(());
    }

    if first_line.starts_with("GET /metrics") {
        scrape_count.fetch_add(1, Ordering::Relaxed);
        let mut body = latest_metrics
            .read()
            .map(|value| value.clone())
            .unwrap_or_else(|_| "# failed to read metrics lock\n".to_string());
        body.push_str("# HELP empireants_server_uptime_seconds HTTP server uptime.\n");
        body.push_str("# TYPE empireants_server_uptime_seconds gauge\n");
        body.push_str(&format!(
            "empireants_server_uptime_seconds {:.6}\n",
            started.elapsed().as_secs_f64()
        ));
        write_response(
            stream,
            200,
            "text/plain; version=0.0.4; charset=utf-8",
            &body,
        )?;
        return Ok(());
    }

    write_response(stream, 404, "text/plain; charset=utf-8", "not found\n")?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Internal Server Error",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.as_bytes().len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

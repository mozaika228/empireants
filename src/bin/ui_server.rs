use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

const DEFAULT_BIND: &str = "127.0.0.1:9110";
const DEFAULT_METRICS_TARGET: &str = "127.0.0.1:9109";

fn main() {
    let bind = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_BIND.to_string());
    let metrics_target =
        env::var("EMPIREANTS_METRICS_TARGET").unwrap_or_else(|_| DEFAULT_METRICS_TARGET.to_string());
    let ui_root = PathBuf::from("ui");

    let listener = match TcpListener::bind(&bind) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("failed to bind ui server on {bind}: {error}");
            std::process::exit(2);
        }
    };

    println!(
        "ui server listening on http://{} (metrics proxy target: {})",
        bind, metrics_target
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = handle_client(&mut stream, &ui_root, &metrics_target) {
                    eprintln!("ui server client error: {error}");
                }
            }
            Err(error) => eprintln!("ui server accept error: {error}"),
        }
    }
}

fn handle_client(stream: &mut TcpStream, ui_root: &Path, metrics_target: &str) -> std::io::Result<()> {
    let mut buffer = [0_u8; 4096];
    let read = stream.read(&mut buffer)?;
    if read == 0 {
        return Ok(());
    }
    let request = String::from_utf8_lossy(&buffer[..read]);
    let first = request.lines().next().unwrap_or_default();

    if first.starts_with("GET /api/metrics") {
        let body = proxy_request(metrics_target, "/metrics").unwrap_or_else(|error| {
            format!("# ui_server proxy error: {}\n", error)
        });
        return write_response(
            stream,
            200,
            "text/plain; version=0.0.4; charset=utf-8",
            body.as_bytes(),
        );
    }

    if first.starts_with("GET /api/healthz") {
        let body = proxy_request(metrics_target, "/healthz").unwrap_or_else(|_| "degraded\n".to_string());
        return write_response(stream, 200, "text/plain; charset=utf-8", body.as_bytes());
    }

    if first.starts_with("GET /") {
        let path = parse_static_path(first);
        let fs_path = ui_root.join(path);
        let fs_path = if fs_path.is_dir() {
            fs_path.join("index.html")
        } else {
            fs_path
        };

        let normalized = fs_path.canonicalize().ok();
        let ui_root_abs = ui_root.canonicalize().ok();
        let allowed = match (normalized.as_ref(), ui_root_abs.as_ref()) {
            (Some(candidate), Some(root)) => candidate.starts_with(root),
            _ => false,
        };
        if !allowed {
            return write_response(stream, 404, "text/plain; charset=utf-8", b"not found\n");
        }

        match fs::read(&fs_path) {
            Ok(body) => {
                let content_type = detect_content_type(&fs_path);
                return write_response(stream, 200, content_type, &body);
            }
            Err(_) => return write_response(stream, 404, "text/plain; charset=utf-8", b"not found\n"),
        }
    }

    write_response(stream, 404, "text/plain; charset=utf-8", b"not found\n")
}

fn parse_static_path(request_line: &str) -> &'static str {
    if request_line.starts_with("GET / ") {
        return "index.html";
    }
    if request_line.starts_with("GET /index.html") {
        return "index.html";
    }
    if request_line.starts_with("GET /styles.css") {
        return "styles.css";
    }
    if request_line.starts_with("GET /app.js") {
        return "app.js";
    }
    "index.html"
}

fn detect_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn proxy_request(target: &str, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(target)?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, target
    );
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let response = String::from_utf8_lossy(&response);

    let mut parts = response.split("\r\n\r\n");
    let _headers = parts.next().unwrap_or_default();
    let body = parts.next().unwrap_or_default();
    Ok(body.to_string())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Internal Server Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

use std::env;
use std::fs;
use std::path::PathBuf;

use empireants::render::build_frame_summary;
use empireants::simulation::{Simulation, SimulationConfig};
use empireants::world::{Cell, Grid};

fn main() {
    let config = SimulationConfig::default();
    let steps = parse_steps();
    let artifact_dir = PathBuf::from("artifacts");

    if let Err(error) = fs::create_dir_all(&artifact_dir) {
        eprintln!("failed to create artifacts directory: {error}");
        std::process::exit(1);
    }

    let mut grid = Grid::new(config.width, config.height);
    seed_world(&mut grid);

    let mut simulation = Simulation::new(config, grid);
    for _ in 0..steps {
        simulation.step();
    }

    let metrics = simulation.metrics();
    let pheromone_path = artifact_dir.join("pheromones.csv");
    let metrics_path = artifact_dir.join("metrics.csv");
    let ant_snapshot_path = artifact_dir.join("ants.csv");
    let prometheus_path = artifact_dir.join("prometheus.prom");

    if let Err(error) = simulation.export_pheromones_csv(&pheromone_path) {
        eprintln!("failed to write pheromone snapshot: {error}");
        std::process::exit(1);
    }

    if let Err(error) = simulation.export_metrics_csv(&metrics_path) {
        eprintln!("failed to write metrics snapshot: {error}");
        std::process::exit(1);
    }

    if let Err(error) = simulation.export_ant_snapshot_csv(&ant_snapshot_path) {
        eprintln!("failed to write ant snapshot: {error}");
        std::process::exit(1);
    }

    if let Err(error) = simulation.export_prometheus(&prometheus_path) {
        eprintln!("failed to write prometheus snapshot: {error}");
        std::process::exit(1);
    }

    println!("EmpireAnts simulation complete");
    println!(
        "steps={} ants={} food_collected={} exploration={} average_decision_score={:.3}",
        metrics.steps,
        metrics.ant_count,
        metrics.food_collected,
        metrics.exploration_moves,
        metrics.average_decision_score
    );
    println!("{}", build_frame_summary(simulation.world(), metrics));
    println!("artifacts:");
    println!("  {}", pheromone_path.display());
    println!("  {}", metrics_path.display());
    println!("  {}", ant_snapshot_path.display());
    println!("  {}", prometheus_path.display());
}

fn parse_steps() -> usize {
    env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(200)
}

fn seed_world(grid: &mut Grid) {
    let cx = grid.width() / 2;
    let cy = grid.height() / 2;
    grid.set_nest(cx, cy);

    for x in 3..8 {
        grid.set_food(x, 4, 24);
    }

    for y in 12..16 {
        grid.set_food(grid.width().saturating_sub(5), y, 18);
    }

    for x in 10..15 {
        grid.set_cell(x, 8, Cell::Obstacle);
    }

    for y in 2..10 {
        grid.set_cell(16, y, Cell::Obstacle);
    }
}

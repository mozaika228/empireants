use crate::simulation::SimulationMetrics;
use crate::world::{Cell, Grid};

pub fn build_frame_summary(grid: &Grid, metrics: SimulationMetrics) -> String {
    let mut obstacles = 0usize;
    let mut food_cells = 0usize;
    for cell in grid.cells() {
        match cell {
            Cell::Obstacle => obstacles += 1,
            Cell::Food(amount) if *amount > 0 => food_cells += 1,
            _ => {}
        }
    }

    format!(
        "frame_summary width={} height={} nest=({}, {}) food_cells={} obstacles={} active_sources={}",
        grid.width(),
        grid.height(),
        grid.nest().x,
        grid.nest().y,
        food_cells,
        obstacles,
        metrics.active_food_sources
    )
}

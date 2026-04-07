use std::time::Instant;

use crate::simulation::{Simulation, SimulationConfig};
use crate::world::{Cell, Grid};

#[derive(Clone, Copy, Debug)]
pub enum ScaleProfile {
    Ant10k,
    Ant100k,
    Ant1m,
}

impl ScaleProfile {
    pub fn from_cli(value: &str) -> Option<Self> {
        match value {
            "10k" => Some(Self::Ant10k),
            "100k" => Some(Self::Ant100k),
            "1m" => Some(Self::Ant1m),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Ant10k => "10k",
            Self::Ant100k => "100k",
            Self::Ant1m => "1m",
        }
    }

    pub fn ant_count(self) -> usize {
        match self {
            Self::Ant10k => 10_000,
            Self::Ant100k => 100_000,
            Self::Ant1m => 1_000_000,
        }
    }

    pub fn default_steps(self) -> usize {
        match self {
            Self::Ant10k => 500,
            Self::Ant100k => 250,
            Self::Ant1m => 120,
        }
    }

    pub fn grid_size(self) -> (usize, usize) {
        match self {
            Self::Ant10k => (192, 192),
            Self::Ant100k => (384, 384),
            Self::Ant1m => (768, 768),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScaleReport {
    pub profile: ScaleProfile,
    pub steps: usize,
    pub ants: usize,
    pub elapsed_seconds: f64,
    pub steps_per_second: f64,
    pub ant_updates_per_second: f64,
    pub estimated_memory_mb: f64,
}

pub fn run_scale_profile(profile: ScaleProfile, override_steps: Option<usize>) -> ScaleReport {
    let ants = profile.ant_count();
    let steps = override_steps.unwrap_or(profile.default_steps()).max(1);
    let (width, height) = profile.grid_size();
    let config = SimulationConfig {
        width,
        height,
        ant_count: ants,
        evaporation_rate: 0.04,
        diffusion_rate: 0.16,
        food_deposit: 0.7,
        home_deposit: 0.5,
        harvest_amount: 1,
    };
    let world = seeded_scale_world(width, height);
    let mut simulation = Simulation::new(config, world);

    let started = Instant::now();
    simulation.run_steps(steps);
    let elapsed_seconds = started.elapsed().as_secs_f64().max(0.000_001);
    let steps_per_second = steps as f64 / elapsed_seconds;
    let ant_updates_per_second = (steps as f64 * ants as f64) / elapsed_seconds;
    let estimated_memory_mb = simulation.estimated_memory_bytes() as f64 / (1024.0 * 1024.0);

    ScaleReport {
        profile,
        steps,
        ants,
        elapsed_seconds,
        steps_per_second,
        ant_updates_per_second,
        estimated_memory_mb,
    }
}

fn seeded_scale_world(width: usize, height: usize) -> Grid {
    let mut grid = Grid::new(width, height);
    grid.set_nest(width / 2, height / 2);

    for y in (height / 8)..(height / 8 + height / 16) {
        for x in 2..(2 + width / 12) {
            grid.set_food(x, y, 64);
        }
    }

    let food_start_x = width.saturating_sub(width / 12 + 2);
    let food_end_x = width.saturating_sub(2);
    let food_start_y = height.saturating_sub(height / 8 + height / 16);
    let food_end_y = height.saturating_sub(height / 8);
    for y in food_start_y..food_end_y {
        for x in food_start_x..food_end_x {
            grid.set_food(x, y, 64);
        }
    }

    // Obstacle bands force path re-routing to avoid trivially open worlds.
    for y in (height / 6)..(height * 5 / 6) {
        let x = width / 3;
        if y % 12 != 0 {
            grid.set_cell(x, y, Cell::Obstacle);
        }
    }

    for x in (width / 5)..(width * 4 / 5) {
        let y = height * 2 / 3;
        if x % 11 != 0 {
            grid.set_cell(x, y, Cell::Obstacle);
        }
    }

    grid
}

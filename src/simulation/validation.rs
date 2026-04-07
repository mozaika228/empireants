use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::simulation::{AcoStrategy, Simulation, SimulationConfig};
use crate::world::{Cell, Grid};

#[derive(Clone, Copy, Debug)]
pub enum ValidationScenario {
    OpenField,
    NarrowPassages,
    ObstacleShift,
}

impl ValidationScenario {
    pub fn all() -> [Self; 3] {
        [Self::OpenField, Self::NarrowPassages, Self::ObstacleShift]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::OpenField => "open_field",
            Self::NarrowPassages => "narrow_passages",
            Self::ObstacleShift => "obstacle_shift",
        }
    }

    fn dimensions(self) -> (usize, usize) {
        match self {
            Self::OpenField => (96, 96),
            Self::NarrowPassages => (112, 96),
            Self::ObstacleShift => (128, 112),
        }
    }

    fn ant_count(self) -> usize {
        match self {
            Self::OpenField => 2_500,
            Self::NarrowPassages => 3_000,
            Self::ObstacleShift => 3_500,
        }
    }

    fn default_steps(self) -> usize {
        match self {
            Self::OpenField => 300,
            Self::NarrowPassages => 360,
            Self::ObstacleShift => 420,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub scenario: ValidationScenario,
    pub strategy: AcoStrategy,
    pub steps: usize,
    pub ants: usize,
    pub food_collected: u32,
    pub first_food_step: usize,
    pub convergence_step: usize,
    pub exploration_efficiency: f64,
    pub throughput_food_per_second: f64,
    pub runtime_stability_score: f64,
}

pub fn run_validation_suite() -> Vec<ValidationResult> {
    run_validation_suite_with(1.0, None)
}

pub fn run_validation_suite_with(
    step_factor: f32,
    ant_cap: Option<usize>,
) -> Vec<ValidationResult> {
    let mut results = Vec::new();
    for scenario in ValidationScenario::all() {
        for strategy in AcoStrategy::all() {
            results.push(run_single(scenario, strategy, step_factor, ant_cap));
        }
    }
    results
}

pub fn export_validation_csv(path: &Path, results: &[ValidationResult]) -> std::io::Result<()> {
    let mut output = String::from(
        "scenario,strategy,steps,ants,food_collected,first_food_step,convergence_step,exploration_efficiency,throughput_food_per_second,runtime_stability_score\n",
    );
    for row in results {
        let _ = writeln!(
            output,
            "{},{},{},{},{},{},{},{:.6},{:.6},{:.6}",
            row.scenario.label(),
            row.strategy.label(),
            row.steps,
            row.ants,
            row.food_collected,
            row.first_food_step,
            row.convergence_step,
            row.exploration_efficiency,
            row.throughput_food_per_second,
            row.runtime_stability_score
        );
    }
    fs::write(path, output)
}

fn run_single(
    scenario: ValidationScenario,
    strategy: AcoStrategy,
    step_factor: f32,
    ant_cap: Option<usize>,
) -> ValidationResult {
    let (width, height) = scenario.dimensions();
    let ants = ant_cap
        .map(|cap| scenario.ant_count().min(cap))
        .unwrap_or_else(|| scenario.ant_count());
    let steps = ((scenario.default_steps() as f32 * step_factor).round() as usize).max(1);
    let mut simulation = Simulation::new(
        SimulationConfig {
            width,
            height,
            ant_count: ants,
            evaporation_rate: 0.05,
            diffusion_rate: 0.17,
            food_deposit: 0.75,
            home_deposit: 0.55,
            harvest_amount: 1,
            aco_strategy: strategy,
        },
        seeded_validation_world(scenario, width, height),
    );

    let mut first_food_step = None;
    let mut convergence_step = None;
    let mut prev_food = 0u32;

    for step in 1..=steps {
        simulation.step();
        let metrics = simulation.metrics();
        if first_food_step.is_none() && metrics.food_collected > 0 {
            first_food_step = Some(step);
        }
        let current_food = metrics.food_collected;
        if convergence_step.is_none() && current_food >= (prev_food + 5) {
            convergence_step = Some(step);
        }
        prev_food = current_food;

        if matches!(scenario, ValidationScenario::ObstacleShift) && step == steps / 2 {
            shift_obstacles(width, height, &mut simulation);
        }
    }

    let metrics = simulation.metrics();
    let elapsed = metrics.simulation_elapsed_seconds.max(0.000_001);
    let exploration_efficiency =
        metrics.food_collected as f64 / (metrics.exploration_moves.max(1) as f64);
    let throughput_food_per_second = metrics.food_collected as f64 / elapsed;
    let instability_penalty = (metrics.runtime_dropped_messages_total
        + metrics.runtime_supervision_events_total
        + metrics.runtime_restarts_total) as f64;
    let runtime_stability_score = 1.0 / (1.0 + instability_penalty);

    ValidationResult {
        scenario,
        strategy,
        steps,
        ants,
        food_collected: metrics.food_collected,
        first_food_step: first_food_step.unwrap_or(steps),
        convergence_step: convergence_step.unwrap_or(steps),
        exploration_efficiency,
        throughput_food_per_second,
        runtime_stability_score,
    }
}

fn seeded_validation_world(scenario: ValidationScenario, width: usize, height: usize) -> Grid {
    let mut grid = Grid::new(width, height);
    grid.set_nest(width / 2, height / 2);

    for x in 3..(3 + width / 12) {
        grid.set_food(x, 3 + height / 10, 48);
    }
    for x in (width - (width / 12 + 3))..(width - 3) {
        grid.set_food(x, height - (3 + height / 10), 48);
    }

    match scenario {
        ValidationScenario::OpenField => {}
        ValidationScenario::NarrowPassages => {
            for y in 8..(height - 8) {
                if y % 9 != 0 {
                    grid.set_cell(width / 3, y, Cell::Obstacle);
                }
                if y % 11 != 0 {
                    grid.set_cell(width * 2 / 3, y, Cell::Obstacle);
                }
            }
        }
        ValidationScenario::ObstacleShift => {
            for x in 8..(width - 8) {
                if x % 10 != 0 {
                    grid.set_cell(x, height / 3, Cell::Obstacle);
                }
                if x % 13 != 0 {
                    grid.set_cell(x, height * 2 / 3, Cell::Obstacle);
                }
            }
        }
    }

    grid
}

fn shift_obstacles(width: usize, height: usize, simulation: &mut Simulation) {
    let world = simulation.world_mut();
    for x in 8..(width - 8) {
        world.set_cell(x, height / 3, Cell::Empty);
        if x % 7 != 0 {
            world.set_cell(x, height / 2, Cell::Obstacle);
        }
    }
}

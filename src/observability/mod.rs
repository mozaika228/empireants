use std::fmt::Write as _;

use crate::simulation::SimulationMetrics;

#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimeSnapshot {
    pub carrying_ants: usize,
    pub searching_ants: usize,
    pub returning_ants: usize,
    pub average_energy: f32,
    pub max_food_pheromone: f32,
    pub max_home_pheromone: f32,
}

pub fn encode_prometheus(metrics: SimulationMetrics, runtime: RuntimeSnapshot) -> String {
    let mut output = String::new();
    output.push_str("# HELP empireants_steps_total Simulation steps executed.\n");
    output.push_str("# TYPE empireants_steps_total counter\n");
    let _ = writeln!(output, "empireants_steps_total {}", metrics.steps);
    output.push_str("# HELP empireants_food_collected_total Food units returned to the nest.\n");
    output.push_str("# TYPE empireants_food_collected_total counter\n");
    let _ = writeln!(
        output,
        "empireants_food_collected_total {}",
        metrics.food_collected
    );
    output.push_str("# HELP empireants_exploration_moves_total Exploratory moves performed by ants.\n");
    output.push_str("# TYPE empireants_exploration_moves_total counter\n");
    let _ = writeln!(
        output,
        "empireants_exploration_moves_total {}",
        metrics.exploration_moves
    );
    output.push_str("# HELP empireants_average_decision_score Mean local decision score.\n");
    output.push_str("# TYPE empireants_average_decision_score gauge\n");
    let _ = writeln!(
        output,
        "empireants_average_decision_score {:.5}",
        metrics.average_decision_score
    );
    output.push_str("# HELP empireants_active_food_sources Active food source cells in the grid.\n");
    output.push_str("# TYPE empireants_active_food_sources gauge\n");
    let _ = writeln!(
        output,
        "empireants_active_food_sources {}",
        metrics.active_food_sources
    );
    output.push_str("# HELP empireants_ants_total Total ants in the colony.\n");
    output.push_str("# TYPE empireants_ants_total gauge\n");
    let _ = writeln!(output, "empireants_ants_total {}", metrics.ant_count);
    output.push_str("# HELP empireants_ants_carrying Number of ants carrying food.\n");
    output.push_str("# TYPE empireants_ants_carrying gauge\n");
    let _ = writeln!(output, "empireants_ants_carrying {}", runtime.carrying_ants);
    output.push_str("# HELP empireants_ants_searching Number of ants searching for food.\n");
    output.push_str("# TYPE empireants_ants_searching gauge\n");
    let _ = writeln!(output, "empireants_ants_searching {}", runtime.searching_ants);
    output.push_str("# HELP empireants_ants_returning Number of ants returning to nest.\n");
    output.push_str("# TYPE empireants_ants_returning gauge\n");
    let _ = writeln!(output, "empireants_ants_returning {}", runtime.returning_ants);
    output.push_str("# HELP empireants_average_energy Mean normalized ant energy.\n");
    output.push_str("# TYPE empireants_average_energy gauge\n");
    let _ = writeln!(output, "empireants_average_energy {:.5}", runtime.average_energy);
    output.push_str("# HELP empireants_pheromone_food_max Maximum food pheromone intensity.\n");
    output.push_str("# TYPE empireants_pheromone_food_max gauge\n");
    let _ = writeln!(
        output,
        "empireants_pheromone_food_max {:.5}",
        runtime.max_food_pheromone
    );
    output.push_str("# HELP empireants_pheromone_home_max Maximum home pheromone intensity.\n");
    output.push_str("# TYPE empireants_pheromone_home_max gauge\n");
    let _ = writeln!(
        output,
        "empireants_pheromone_home_max {:.5}",
        runtime.max_home_pheromone
    );
    output
}

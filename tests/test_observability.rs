use empireants::observability::{encode_prometheus, RuntimeSnapshot};
use empireants::simulation::SimulationMetrics;

#[test]
fn prometheus_encoder_exposes_expected_metrics() {
    let metrics = SimulationMetrics {
        steps: 120,
        ant_count: 256,
        food_collected: 42,
        exploration_moves: 900,
        average_decision_score: 0.75,
        active_food_sources: 3,
    };
    let runtime = RuntimeSnapshot {
        carrying_ants: 12,
        searching_ants: 200,
        returning_ants: 56,
        average_energy: 0.91,
        max_food_pheromone: 1.8,
        max_home_pheromone: 1.2,
    };

    let output = encode_prometheus(metrics, runtime);

    assert!(output.contains("empireants_steps_total 120"));
    assert!(output.contains("empireants_food_collected_total 42"));
    assert!(output.contains("empireants_ants_carrying 12"));
    assert!(output.contains("empireants_average_energy 0.91000"));
}

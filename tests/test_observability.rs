use empireants::observability::{
    encode_prometheus, encode_prometheus_with_metadata, RuntimeSnapshot, ScrapeMetadata,
};
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
        last_step_micros: 812,
        average_step_micros: 902.5,
        max_step_micros: 1440,
        simulation_elapsed_seconds: 0.108,
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
    assert!(output.contains("empireants_step_latency_last_microseconds 812"));
}

#[test]
fn prometheus_encoder_supports_scrape_metadata() {
    let metrics = SimulationMetrics {
        steps: 10,
        ant_count: 16,
        food_collected: 3,
        exploration_moves: 27,
        average_decision_score: 0.44,
        active_food_sources: 2,
        last_step_micros: 100,
        average_step_micros: 97.2,
        max_step_micros: 160,
        simulation_elapsed_seconds: 0.04,
    };
    let runtime = RuntimeSnapshot::default();
    let metadata = ScrapeMetadata {
        uptime_seconds: 15.2,
        scrape_count: 4,
    };

    let output = encode_prometheus_with_metadata(metrics, runtime, metadata);
    assert!(output.contains("empireants_uptime_seconds 15.200000"));
    assert!(output.contains("empireants_metrics_scrapes_total 4"));
}

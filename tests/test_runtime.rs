use empireants::ant::{ActorRuntime, Ant, RuntimeConfig};
use empireants::simulation::AcoPolicy;
use empireants::world::{Grid, PheromoneField, Position};

#[test]
fn runtime_applies_backpressure_when_mailbox_is_full() {
    let mut runtime = ActorRuntime::new(RuntimeConfig {
        mailbox_capacity: 1,
        max_restarts_per_tick: 10,
    });
    let ants = vec![
        Ant::new(0, Position { x: 2, y: 2 }),
        Ant::new(1, Position { x: 3, y: 2 }),
    ];
    let grid = Grid::new(8, 8);
    let pheromones = PheromoneField::new(8, 8);
    let policy = AcoPolicy::default();

    let updates = runtime.gather_updates(&ants, &grid, &pheromones, &policy, 0);
    let stats = runtime.stats();

    assert_eq!(updates.len(), 1);
    assert!(stats.dropped_messages_total >= 1);
    assert!(stats.supervision_events_total >= 1);
}

use crate::ant::agent::AntDecision;
use crate::ant::{Ant, AntId, AntState};
use crate::simulation::AcoPolicy;
use crate::world::{Grid, PheromoneField, Position};
use std::panic::AssertUnwindSafe;

#[derive(Clone, Copy, Debug)]
pub struct AntUpdate {
    pub ant_id: AntId,
    pub from: Position,
    pub to: Position,
    pub state: AntState,
    pub carrying_food: bool,
    pub decision_score: f32,
    pub exploratory: bool,
    pub recovered: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct RuntimeConfig {
    pub mailbox_capacity: usize,
    pub max_restarts_per_tick: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            mailbox_capacity: 2_000_000,
            max_restarts_per_tick: 10_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimeStats {
    pub mailbox_len: usize,
    pub dropped_messages_total: u64,
    pub dropped_messages_tick: u64,
    pub restarts_total: u64,
    pub restarts_tick: u64,
    pub supervision_events_total: u64,
    pub processed_messages_tick: u64,
}

#[derive(Clone, Copy, Debug)]
enum RuntimeCommand {
    Decide { ant_id: AntId, tick: usize },
    Recover { ant_id: AntId },
}

pub struct ActorRuntime {
    config: RuntimeConfig,
    mailbox: Vec<RuntimeCommand>,
    stats: RuntimeStats,
}

impl Default for ActorRuntime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

impl ActorRuntime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            mailbox: Vec::new(),
            stats: RuntimeStats::default(),
        }
    }

    pub fn stats(&self) -> RuntimeStats {
        self.stats
    }

    pub fn gather_updates(
        &mut self,
        ants: &[Ant],
        grid: &Grid,
        pheromones: &PheromoneField,
        policy: &AcoPolicy,
        tick: usize,
    ) -> Vec<AntUpdate> {
        self.reset_tick_counters();
        self.mailbox.clear();
        self.mailbox.reserve(ants.len());

        for ant in ants {
            self.try_enqueue(RuntimeCommand::Decide {
                ant_id: ant.id,
                tick,
            });
        }

        let mut updates = Vec::with_capacity(self.mailbox.len());
        let mut cursor = 0usize;
        while cursor < self.mailbox.len() {
            let command = self.mailbox[cursor];
            cursor += 1;
            self.stats.processed_messages_tick += 1;

            match command {
                RuntimeCommand::Decide { ant_id, tick } => {
                    let Some(ant) = ants.get(ant_id) else {
                        self.register_supervision_event();
                        continue;
                    };

                    let safe_decision = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        ant.decide(grid, pheromones, policy, tick)
                    }));
                    match safe_decision {
                        Ok(AntDecision {
                            next,
                            pheromone_score,
                            was_exploratory,
                        }) if pheromone_score.is_finite() => {
                            updates.push(AntUpdate {
                                ant_id: ant.id,
                                from: ant.position,
                                to: next,
                                state: ant.state,
                                carrying_food: ant.carrying_food,
                                decision_score: pheromone_score,
                                exploratory: was_exploratory,
                                recovered: false,
                            });
                        }
                        _ => {
                            self.register_supervision_event();
                            self.try_enqueue(RuntimeCommand::Recover { ant_id: ant.id });
                        }
                    }
                }
                RuntimeCommand::Recover { ant_id } => {
                    if self.stats.restarts_tick >= self.config.max_restarts_per_tick as u64 {
                        self.register_supervision_event();
                        continue;
                    }
                    let Some(ant) = ants.get(ant_id) else {
                        self.register_supervision_event();
                        continue;
                    };
                    self.stats.restarts_tick += 1;
                    self.stats.restarts_total += 1;
                    let nest = grid.nest();
                    updates.push(AntUpdate {
                        ant_id: ant.id,
                        from: ant.position,
                        to: nest,
                        state: AntState::Searching,
                        carrying_food: false,
                        decision_score: 0.0,
                        exploratory: false,
                        recovered: true,
                    });
                }
            }
        }

        self.stats.mailbox_len = self.mailbox.len();
        updates
    }

    fn try_enqueue(&mut self, command: RuntimeCommand) {
        if self.mailbox.len() >= self.config.mailbox_capacity {
            self.stats.dropped_messages_tick += 1;
            self.stats.dropped_messages_total += 1;
            self.register_supervision_event();
            return;
        }
        self.mailbox.push(command);
    }

    fn register_supervision_event(&mut self) {
        self.stats.supervision_events_total += 1;
    }

    fn reset_tick_counters(&mut self) {
        self.stats.dropped_messages_tick = 0;
        self.stats.restarts_tick = 0;
        self.stats.processed_messages_tick = 0;
    }
}

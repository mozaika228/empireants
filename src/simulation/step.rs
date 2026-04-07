use std::fmt::Write as _;
use std::fs;
use std::mem::size_of;
use std::path::Path;
use std::time::Instant;

use crate::ant::{ActorRuntime, Ant, AntState};
use crate::observability::{encode_prometheus, RuntimeSnapshot};
use crate::simulation::AcoPolicy;
use crate::world::{Cell, Grid, PheromoneField};

#[derive(Clone, Debug)]
pub struct SimulationConfig {
    pub width: usize,
    pub height: usize,
    pub ant_count: usize,
    pub evaporation_rate: f32,
    pub diffusion_rate: f32,
    pub food_deposit: f32,
    pub home_deposit: f32,
    pub harvest_amount: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            width: 24,
            height: 18,
            ant_count: 256,
            evaporation_rate: 0.06,
            diffusion_rate: 0.18,
            food_deposit: 0.8,
            home_deposit: 0.6,
            harvest_amount: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SimulationMetrics {
    pub steps: usize,
    pub ant_count: usize,
    pub food_collected: u32,
    pub exploration_moves: u64,
    pub average_decision_score: f32,
    pub active_food_sources: usize,
    pub last_step_micros: u64,
    pub average_step_micros: f64,
    pub max_step_micros: u64,
    pub simulation_elapsed_seconds: f64,
    pub runtime_mailbox_len: usize,
    pub runtime_dropped_messages_total: u64,
    pub runtime_restarts_total: u64,
    pub runtime_supervision_events_total: u64,
}

pub struct Simulation {
    config: SimulationConfig,
    grid: Grid,
    pheromones: PheromoneField,
    ants: Vec<Ant>,
    runtime: ActorRuntime,
    aco: AcoPolicy,
    metrics: SimulationMetrics,
}

impl Simulation {
    pub fn new(config: SimulationConfig, grid: Grid) -> Self {
        let nest = grid.nest();
        let ants = (0..config.ant_count)
            .map(|id| Ant::new(id, nest))
            .collect::<Vec<_>>();
        Self {
            pheromones: PheromoneField::new(config.width, config.height),
            runtime: ActorRuntime,
            aco: AcoPolicy::default(),
            metrics: SimulationMetrics {
                ant_count: config.ant_count,
                active_food_sources: grid.count_food_cells(),
                ..SimulationMetrics::default()
            },
            config,
            grid,
            ants,
        }
    }

    pub fn step(&mut self) {
        let started = Instant::now();
        let tick = self.metrics.steps;
        let updates = self
            .runtime
            .gather_updates(&self.ants, &self.grid, &self.pheromones, &self.aco, tick);

        let mut score_sum = 0.0;
        for update in updates {
            let ant = &mut self.ants[update.ant_id];
            if update.recovered {
                ant.position = update.to;
                ant.last_position = Some(update.from);
                ant.state = update.state;
                ant.carrying_food = update.carrying_food;
                ant.energy = 1.0;
                continue;
            }
            ant.move_to(update.to);
            score_sum += update.decision_score.max(0.0);

            if update.exploratory {
                self.metrics.exploration_moves += 1;
            }

            if update.carrying_food {
                self.pheromones.deposit_food(update.from, self.config.food_deposit);
                if update.to == self.grid.nest() {
                    ant.carrying_food = false;
                    ant.state = AntState::Searching;
                    ant.energy = (ant.energy + 0.35).min(1.0);
                    self.metrics.food_collected += 1;
                }
            } else {
                self.pheromones.deposit_home(update.from, self.config.home_deposit);
                if let Some(Cell::Food(_)) = self.grid.get(update.to) {
                    let harvested = self.grid.harvest_food(update.to, self.config.harvest_amount);
                    if harvested > 0 {
                        ant.carrying_food = true;
                        ant.state = AntState::Returning;
                    }
                }
            }
        }

        self.pheromones.evaporate_and_diffuse(
            &self.grid,
            self.config.evaporation_rate,
            self.config.diffusion_rate,
        );
        self.metrics.steps += 1;
        self.metrics.active_food_sources = self.grid.count_food_cells();
        self.metrics.average_decision_score = if self.ants.is_empty() {
            0.0
        } else {
            score_sum / self.ants.len() as f32
        };

        let step_micros = started.elapsed().as_micros() as u64;
        self.metrics.last_step_micros = step_micros;
        self.metrics.max_step_micros = self.metrics.max_step_micros.max(step_micros);
        if self.metrics.steps == 1 {
            self.metrics.average_step_micros = step_micros as f64;
        } else {
            let prev_weight = (self.metrics.steps - 1) as f64;
            self.metrics.average_step_micros =
                (self.metrics.average_step_micros * prev_weight + step_micros as f64)
                    / self.metrics.steps as f64;
        }
        self.metrics.simulation_elapsed_seconds += step_micros as f64 / 1_000_000.0;
        let runtime_stats = self.runtime.stats();
        self.metrics.runtime_mailbox_len = runtime_stats.mailbox_len;
        self.metrics.runtime_dropped_messages_total = runtime_stats.dropped_messages_total;
        self.metrics.runtime_restarts_total = runtime_stats.restarts_total;
        self.metrics.runtime_supervision_events_total = runtime_stats.supervision_events_total;
    }

    pub fn run_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn world(&self) -> &Grid {
        &self.grid
    }

    pub fn pheromones(&self) -> &PheromoneField {
        &self.pheromones
    }

    pub fn ants(&self) -> &[Ant] {
        &self.ants
    }

    pub fn metrics(&self) -> SimulationMetrics {
        self.metrics
    }

    pub fn runtime_snapshot(&self) -> RuntimeSnapshot {
        let mut carrying_ants = 0usize;
        let mut searching_ants = 0usize;
        let mut returning_ants = 0usize;
        let mut total_energy = 0.0f32;

        for ant in &self.ants {
            total_energy += ant.energy;
            if ant.carrying_food {
                carrying_ants += 1;
            }
            match ant.state {
                AntState::Searching => searching_ants += 1,
                AntState::Returning => returning_ants += 1,
            }
        }

        let average_energy = if self.ants.is_empty() {
            0.0
        } else {
            total_energy / self.ants.len() as f32
        };

        RuntimeSnapshot {
            carrying_ants,
            searching_ants,
            returning_ants,
            average_energy,
            max_food_pheromone: self.pheromones.max_food(),
            max_home_pheromone: self.pheromones.max_home(),
        }
    }

    pub fn export_pheromones_csv(&self, path: &Path) -> std::io::Result<()> {
        let mut output = String::from("x,y,food,home\n");
        for (x, y, food, home) in self.pheromones.to_rows() {
            let _ = writeln!(output, "{x},{y},{food:.5},{home:.5}");
        }
        fs::write(path, output)
    }

    pub fn export_metrics_csv(&self, path: &Path) -> std::io::Result<()> {
        let metrics = self.metrics();
        let output = format!(
            "steps,ant_count,food_collected,exploration_moves,average_decision_score,active_food_sources,last_step_micros,average_step_micros,max_step_micros,simulation_elapsed_seconds,runtime_mailbox_len,runtime_dropped_messages_total,runtime_restarts_total,runtime_supervision_events_total\n{},{},{},{},{:.5},{},{},{:.2},{},{},{},{},{},{}\n",
            metrics.steps,
            metrics.ant_count,
            metrics.food_collected,
            metrics.exploration_moves,
            metrics.average_decision_score,
            metrics.active_food_sources,
            metrics.last_step_micros,
            metrics.average_step_micros,
            metrics.max_step_micros,
            metrics.simulation_elapsed_seconds,
            metrics.runtime_mailbox_len,
            metrics.runtime_dropped_messages_total,
            metrics.runtime_restarts_total,
            metrics.runtime_supervision_events_total
        );
        fs::write(path, output)
    }

    pub fn export_ant_snapshot_csv(&self, path: &Path) -> std::io::Result<()> {
        let mut output = String::from("id,x,y,state,carrying_food,energy\n");
        for ant in &self.ants {
            let state = match ant.state {
                AntState::Searching => "searching",
                AntState::Returning => "returning",
            };
            let _ = writeln!(
                output,
                "{},{},{},{},{},{}",
                ant.id, ant.position.x, ant.position.y, state, ant.carrying_food, ant.energy
            );
        }
        fs::write(path, output)
    }

    pub fn export_prometheus(&self, path: &Path) -> std::io::Result<()> {
        let output = encode_prometheus(self.metrics(), self.runtime_snapshot());
        fs::write(path, output)
    }

    pub fn estimated_memory_bytes(&self) -> usize {
        let ant_bytes = self.ants.capacity() * size_of::<Ant>();
        let cell_bytes = self.grid.cells().len() * size_of::<Cell>();
        let pheromone_channel_bytes = self.grid.width() * self.grid.height() * size_of::<f32>() * 2;
        ant_bytes + cell_bytes + pheromone_channel_bytes
    }
}

use crate::simulation::AcoPolicy;
use crate::world::{Cell, Grid, PheromoneField, Position};

pub type AntId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AntState {
    Searching,
    Returning,
}

#[derive(Clone, Debug)]
pub struct Ant {
    pub id: AntId,
    pub position: Position,
    pub carrying_food: bool,
    pub energy: f32,
    pub state: AntState,
    pub last_position: Option<Position>,
}

#[derive(Clone, Copy, Debug)]
pub struct AntDecision {
    pub next: Position,
    pub pheromone_score: f32,
    pub was_exploratory: bool,
}

impl Ant {
    pub fn new(id: AntId, position: Position) -> Self {
        Self {
            id,
            position,
            carrying_food: false,
            energy: 1.0,
            state: AntState::Searching,
            last_position: None,
        }
    }

    pub fn decide(
        &self,
        grid: &Grid,
        pheromones: &PheromoneField,
        policy: &AcoPolicy,
        tick: usize,
    ) -> AntDecision {
        let neighbors = grid
            .neighbors4(self.position)
            .into_iter()
            .filter(|position| grid.is_walkable(*position))
            .collect::<Vec<_>>();

        if neighbors.is_empty() {
            return AntDecision {
                next: self.position,
                pheromone_score: 0.0,
                was_exploratory: false,
            };
        }

        let mut best = neighbors[0];
        let mut best_score = f32::MIN;
        let mut exploratory = false;

        for candidate in neighbors {
            let snapshot = pheromones.snapshot(candidate);
            let trail = if self.carrying_food {
                snapshot.home
            } else {
                snapshot.food
            };

            let distance_bias = if self.carrying_food {
                1.0 / (grid.distance_to_nest(candidate) as f32 + 1.0)
            } else {
                match grid.get(candidate) {
                    Some(Cell::Food(amount)) if amount > 0 => 2.0,
                    _ => 1.0 / (grid.distance_to_nest(candidate) as f32 + 1.0),
                }
            };

            let revisit_penalty = if self.last_position == Some(candidate) {
                0.15
            } else {
                1.0
            };

            let score = policy.score_candidate(trail, distance_bias, revisit_penalty);
            if score > best_score {
                best = candidate;
                best_score = score;
                exploratory = trail < policy.exploration_threshold(tick, self.id);
            }
        }

        AntDecision {
            next: best,
            pheromone_score: best_score,
            was_exploratory: exploratory,
        }
    }

    pub fn move_to(&mut self, next: Position) {
        self.last_position = Some(self.position);
        self.position = next;
        self.energy = (self.energy - 0.002).max(0.2);
    }
}

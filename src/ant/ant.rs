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
        let x = self.position.x;
        let y = self.position.y;
        let mut best = self.position;
        let mut best_score = f32::MIN;
        let mut found = false;
        let mut exploratory = false;

        if x > 0 {
            evaluate_candidate(
                self,
                Position { x: x - 1, y },
                grid,
                pheromones,
                policy,
                tick,
                &mut best,
                &mut best_score,
                &mut exploratory,
                &mut found,
            );
        }
        if x + 1 < grid.width() {
            evaluate_candidate(
                self,
                Position { x: x + 1, y },
                grid,
                pheromones,
                policy,
                tick,
                &mut best,
                &mut best_score,
                &mut exploratory,
                &mut found,
            );
        }
        if y > 0 {
            evaluate_candidate(
                self,
                Position { x, y: y - 1 },
                grid,
                pheromones,
                policy,
                tick,
                &mut best,
                &mut best_score,
                &mut exploratory,
                &mut found,
            );
        }
        if y + 1 < grid.height() {
            evaluate_candidate(
                self,
                Position { x, y: y + 1 },
                grid,
                pheromones,
                policy,
                tick,
                &mut best,
                &mut best_score,
                &mut exploratory,
                &mut found,
            );
        }

        if !found {
            return AntDecision {
                next: self.position,
                pheromone_score: 0.0,
                was_exploratory: false,
            };
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

#[allow(clippy::too_many_arguments)]
fn evaluate_candidate(
    ant: &Ant,
    candidate: Position,
    grid: &Grid,
    pheromones: &PheromoneField,
    policy: &AcoPolicy,
    tick: usize,
    best: &mut Position,
    best_score: &mut f32,
    exploratory: &mut bool,
    found: &mut bool,
) {
    if !grid.is_walkable(candidate) {
        return;
    }

    let snapshot = pheromones.snapshot(candidate);
    let trail = if ant.carrying_food {
        snapshot.home
    } else {
        snapshot.food
    };

    let distance_bias = if ant.carrying_food {
        1.0 / (grid.distance_to_nest(candidate) as f32 + 1.0)
    } else {
        match grid.get(candidate) {
            Some(Cell::Food(amount)) if amount > 0 => 2.0,
            _ => 1.0 / (grid.distance_to_nest(candidate) as f32 + 1.0),
        }
    };

    let revisit_penalty = if ant.last_position == Some(candidate) {
        0.15
    } else {
        1.0
    };

    let score = policy.score_candidate(trail, distance_bias, revisit_penalty);
    if score > *best_score {
        *best = candidate;
        *best_score = score;
        *exploratory = trail < policy.exploration_threshold(tick, ant.id);
        *found = true;
    }
}

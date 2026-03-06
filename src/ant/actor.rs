use crate::ant::ant::AntDecision;
use crate::ant::{Ant, AntId, AntState};
use crate::simulation::AcoPolicy;
use crate::world::{Grid, PheromoneField, Position};

#[derive(Clone, Copy, Debug)]
pub struct AntUpdate {
    pub ant_id: AntId,
    pub from: Position,
    pub to: Position,
    pub state: AntState,
    pub carrying_food: bool,
    pub decision_score: f32,
    pub exploratory: bool,
}

#[derive(Default)]
pub struct ActorRuntime;

impl ActorRuntime {
    pub fn gather_updates(
        &self,
        ants: &[Ant],
        grid: &Grid,
        pheromones: &PheromoneField,
        policy: &AcoPolicy,
        tick: usize,
    ) -> Vec<AntUpdate> {
        ants.iter()
            .map(|ant| {
                let AntDecision {
                    next,
                    pheromone_score,
                    was_exploratory,
                } = ant.decide(grid, pheromones, policy, tick);
                AntUpdate {
                    ant_id: ant.id,
                    from: ant.position,
                    to: next,
                    state: ant.state,
                    carrying_food: ant.carrying_food,
                    decision_score: pheromone_score,
                    exploratory: was_exploratory,
                }
            })
            .collect()
    }
}

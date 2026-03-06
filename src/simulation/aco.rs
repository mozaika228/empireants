#[derive(Clone, Copy, Debug)]
pub enum AcoStrategy {
    Basic,
    MaxMin,
    AsRank,
    AntNet,
}

#[derive(Clone, Copy, Debug)]
pub struct AcoPolicy {
    alpha: f32,
    beta: f32,
    max_trail: f32,
    min_trail: f32,
    strategy: AcoStrategy,
}

impl AcoPolicy {
    pub fn new(strategy: AcoStrategy) -> Self {
        let (alpha, beta, max_trail, min_trail) = match strategy {
            AcoStrategy::Basic => (1.0, 2.0, 4.0, 0.05),
            AcoStrategy::MaxMin => (1.4, 2.2, 3.0, 0.10),
            AcoStrategy::AsRank => (1.2, 2.8, 4.5, 0.08),
            AcoStrategy::AntNet => (1.5, 1.6, 5.0, 0.05),
        };
        Self {
            alpha,
            beta,
            max_trail,
            min_trail,
            strategy,
        }
    }

    pub fn score_candidate(&self, pheromone: f32, heuristic: f32, revisit_penalty: f32) -> f32 {
        let bounded = pheromone.clamp(self.min_trail, self.max_trail);
        let base = bounded.powf(self.alpha) * heuristic.powf(self.beta) * revisit_penalty;
        match self.strategy {
            AcoStrategy::Basic => base,
            AcoStrategy::MaxMin => base.min(self.max_trail),
            AcoStrategy::AsRank => base * 1.1,
            AcoStrategy::AntNet => base + heuristic * 0.25,
        }
    }

    pub fn exploration_threshold(&self, tick: usize, ant_id: usize) -> f32 {
        let hash = ((tick as u64 * 1103515245) ^ ant_id as u64).wrapping_add(12345);
        (hash % 1000) as f32 / 1000.0
    }
}

impl Default for AcoPolicy {
    fn default() -> Self {
        Self::new(AcoStrategy::MaxMin)
    }
}


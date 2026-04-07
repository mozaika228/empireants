use super::{Grid, Position};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PheromoneSnapshot {
    pub food: f32,
    pub home: f32,
}

#[derive(Clone, Debug)]
pub struct PheromoneField {
    width: usize,
    height: usize,
    food: Vec<f32>,
    home: Vec<f32>,
}

impl PheromoneField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            food: vec![0.0; width * height],
            home: vec![0.0; width * height],
        }
    }

    pub fn snapshot(&self, position: Position) -> PheromoneSnapshot {
        let idx = self.index(position);
        PheromoneSnapshot {
            food: self.food[idx],
            home: self.home[idx],
        }
    }

    pub fn deposit_food(&mut self, position: Position, amount: f32) {
        let idx = self.index(position);
        self.food[idx] += amount.max(0.0);
    }

    pub fn deposit_home(&mut self, position: Position, amount: f32) {
        let idx = self.index(position);
        self.home[idx] += amount.max(0.0);
    }

    pub fn evaporate_and_diffuse(&mut self, grid: &Grid, evaporation: f32, diffusion: f32) {
        let evaporation = evaporation.clamp(0.0, 1.0);
        let diffusion = diffusion.clamp(0.0, 0.25);
        self.food = diffuse_channel(&self.food, grid, self.width, self.height, evaporation, diffusion);
        self.home = diffuse_channel(&self.home, grid, self.width, self.height, evaporation, diffusion);
    }

    pub fn max_food(&self) -> f32 {
        self.food.iter().copied().fold(0.0, f32::max)
    }

    pub fn max_home(&self) -> f32 {
        self.home.iter().copied().fold(0.0, f32::max)
    }

    pub fn to_rows(&self) -> Vec<(usize, usize, f32, f32)> {
        let mut rows = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                rows.push((x, y, self.food[idx], self.home[idx]));
            }
        }
        rows
    }

    fn index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }
}

fn diffuse_channel(
    channel: &[f32],
    grid: &Grid,
    width: usize,
    height: usize,
    evaporation: f32,
    diffusion: f32,
) -> Vec<f32> {
    let mut next = vec![0.0; width * height];
    for y in 0..height {
        for x in 0..width {
            let position = Position { x, y };
            if !grid.is_walkable(position) {
                continue;
            }

            let idx = y * width + x;
            let value = channel[idx] * (1.0 - evaporation);
            let retained = value * (1.0 - diffusion);
            next[idx] += retained;

            let mut walkable_neighbors = 0usize;
            if x > 0 && grid.is_walkable(Position { x: x - 1, y }) {
                walkable_neighbors += 1;
            }
            if x + 1 < width && grid.is_walkable(Position { x: x + 1, y }) {
                walkable_neighbors += 1;
            }
            if y > 0 && grid.is_walkable(Position { x, y: y - 1 }) {
                walkable_neighbors += 1;
            }
            if y + 1 < height && grid.is_walkable(Position { x, y: y + 1 }) {
                walkable_neighbors += 1;
            }

            if walkable_neighbors == 0 {
                continue;
            }

            let share = value * diffusion / walkable_neighbors as f32;
            if x > 0 && grid.is_walkable(Position { x: x - 1, y }) {
                next[y * width + (x - 1)] += share;
            }
            if x + 1 < width && grid.is_walkable(Position { x: x + 1, y }) {
                next[y * width + (x + 1)] += share;
            }
            if y > 0 && grid.is_walkable(Position { x, y: y - 1 }) {
                next[(y - 1) * width + x] += share;
            }
            if y + 1 < height && grid.is_walkable(Position { x, y: y + 1 }) {
                next[(y + 1) * width + x] += share;
            }
        }
    }
    next
}

use empireants::world::{Grid, PheromoneField, Position};

#[test]
fn evaporation_reduces_total_pheromone() {
    let grid = Grid::new(8, 8);
    let mut field = PheromoneField::new(8, 8);
    field.deposit_food(Position { x: 3, y: 3 }, 10.0);

    let before = field.snapshot(Position { x: 3, y: 3 }).food;
    field.evaporate_and_diffuse(&grid, 0.1, 0.0);
    let after = field.snapshot(Position { x: 3, y: 3 }).food;

    assert!(after < before);
}

#[test]
fn diffusion_spreads_signal_to_neighbors() {
    let grid = Grid::new(8, 8);
    let mut field = PheromoneField::new(8, 8);
    let center = Position { x: 4, y: 4 };
    let neighbor = Position { x: 4, y: 5 };
    field.deposit_home(center, 8.0);

    field.evaporate_and_diffuse(&grid, 0.0, 0.2);
    assert!(field.snapshot(neighbor).home > 0.0);
}


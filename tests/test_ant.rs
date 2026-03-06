use empireants::ant::{Ant, AntState};
use empireants::simulation::AcoPolicy;
use empireants::world::{Grid, PheromoneField, Position};

#[test]
fn ant_prefers_food_signal_while_searching() {
    let grid = Grid::new(8, 8);
    let mut field = PheromoneField::new(8, 8);
    let ant = Ant::new(0, Position { x: 4, y: 4 });

    field.deposit_food(Position { x: 5, y: 4 }, 2.0);
    field.deposit_food(Position { x: 4, y: 5 }, 0.2);

    let decision = ant.decide(&grid, &field, &AcoPolicy::default(), 1);
    assert_eq!(decision.next, Position { x: 5, y: 4 });
}

#[test]
fn ant_returns_home_when_carrying_food() {
    let mut grid = Grid::new(8, 8);
    grid.set_nest(1, 1);
    let mut ant = Ant::new(0, Position { x: 5, y: 5 });
    ant.carrying_food = true;
    ant.state = AntState::Returning;
    let field = PheromoneField::new(8, 8);

    let decision = ant.decide(&grid, &field, &AcoPolicy::default(), 1);
    assert!(grid.distance_to_nest(decision.next) < grid.distance_to_nest(ant.position));
}


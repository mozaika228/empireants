#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Obstacle,
    Nest,
    Food(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    nest: Position,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 2 && height > 2, "grid must be larger than 2x2");
        let nest = Position {
            x: width / 2,
            y: height / 2,
        };
        let mut cells = vec![Cell::Empty; width * height];
        cells[nest.y * width + nest.x] = Cell::Nest;
        Self {
            width,
            height,
            cells,
            nest,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn nest(&self) -> Position {
        self.nest
    }

    pub fn is_within_bounds(&self, position: Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn get(&self, position: Position) -> Option<Cell> {
        self.is_within_bounds(position)
            .then(|| self.cells[self.index(position)])
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let position = Position { x, y };
        if self.is_within_bounds(position) {
            let index = self.index(position);
            self.cells[index] = cell;
        }
    }

    pub fn set_nest(&mut self, x: usize, y: usize) {
        let position = Position { x, y };
        if !self.is_within_bounds(position) {
            return;
        }
        let old = self.index(self.nest);
        self.cells[old] = Cell::Empty;
        self.nest = position;
        let new = self.index(position);
        self.cells[new] = Cell::Nest;
    }

    pub fn set_food(&mut self, x: usize, y: usize, amount: u32) {
        self.set_cell(x, y, Cell::Food(amount));
    }

    pub fn neighbors4(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(4);
        if position.x > 0 {
            neighbors.push(Position {
                x: position.x - 1,
                y: position.y,
            });
        }
        if position.x + 1 < self.width {
            neighbors.push(Position {
                x: position.x + 1,
                y: position.y,
            });
        }
        if position.y > 0 {
            neighbors.push(Position {
                x: position.x,
                y: position.y - 1,
            });
        }
        if position.y + 1 < self.height {
            neighbors.push(Position {
                x: position.x,
                y: position.y + 1,
            });
        }
        neighbors
    }

    pub fn is_walkable(&self, position: Position) -> bool {
        !matches!(self.get(position), Some(Cell::Obstacle) | None)
    }

    pub fn distance_to_nest(&self, position: Position) -> usize {
        position.x.abs_diff(self.nest.x) + position.y.abs_diff(self.nest.y)
    }

    pub fn harvest_food(&mut self, position: Position, amount: u32) -> u32 {
        let Some(Cell::Food(available)) = self.get(position) else {
            return 0;
        };

        let harvested = available.min(amount);
        let remaining = available - harvested;
        self.set_cell(
            position.x,
            position.y,
            if remaining == 0 {
                Cell::Empty
            } else {
                Cell::Food(remaining)
            },
        );
        harvested
    }

    pub fn count_food_cells(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| matches!(cell, Cell::Food(amount) if *amount > 0))
            .count()
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    fn index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }
}


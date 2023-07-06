pub struct Cell {
    pub pos: (i32, i32),
    pub alive: bool,
    pub neighbors: Vec<Cell>,
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell {
            pos: (x, y),
            alive,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: Cell) {
        self.neighbors.push(neighbor);
    }

    pub fn check_is_alive(&mut self) {
        let mut alive_neighbors = 0;

        for neighbor in &self.neighbors {
            if neighbor.alive {
                alive_neighbors += 1;
            }
        }

        if self.alive {
            if alive_neighbors < 2 || alive_neighbors > 3 {
                self.alive = false;
            }
        } else {
            if alive_neighbors == 3 {
                self.alive = true;
            }
        }
    }
}
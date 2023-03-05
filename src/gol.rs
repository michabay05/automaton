#[derive(Copy, Clone, PartialEq)]
enum GolCells {
    Dead,
    Alive,
}

// The name 'gol' is an acronym for 'Game of Life'
pub struct Gol {
    cells: Vec<GolCells>,
    x_count: usize,
    y_count: usize,
    cell_size: (f32, f32),
    pub paused: bool,
}

impl Gol {
    pub fn new(x_count: usize, y_count: usize) -> Self {
        Self {
            cells: vec![GolCells::Dead; x_count * y_count],
            x_count,
            y_count,
            cell_size: (0.0, 0.0),
            paused: false,
        }
    }

    pub fn get_x_count(&self) -> usize {
        self.x_count
    }

    pub fn get_y_count(&self) -> usize {
        self.y_count
    }

    pub fn randomize(&mut self) {
        for col in 0..self.get_x_count() {
            for row in 0..self.get_y_count() {
                if rand::random::<f32>() < 0.2 {
                    self.toggle(col, row);
                }
            }
        }
    }

    pub fn restart(&mut self) {
        for el in &mut self.cells {
            *el = GolCells::Dead;
        }
    }

    pub fn is_alive(&self, col: usize, row: usize) -> bool {
        let ind = self.coord_to_ind(col, row);
        self.cells[ind] == GolCells::Alive
    }

    pub fn update(&mut self) {
        let mut to_be_alive: Vec<usize> = vec![];
        let mut to_be_dead: Vec<usize> = vec![];
        for col in 0..self.x_count {
            for row in 0..self.y_count {
                let ind = self.coord_to_ind(col, row);
                if self.apply_rule(col, row) {
                    to_be_alive.push(ind);
                } else {
                    to_be_dead.push(ind);
                }
            }
        }

        for ind in to_be_alive {
            self.cells[ind] = GolCells::Alive;
        }

        for ind in to_be_dead {
            self.cells[ind] = GolCells::Dead;
        }
    }

    pub fn toggle(&mut self, col: usize, row: usize) {
        let ind = self.coord_to_ind(col, row);
        if self.cells[ind] == GolCells::Alive {
            self.cells[ind] = GolCells::Dead;
        } else {
            self.cells[ind] = GolCells::Alive;
        }
    }

    fn apply_rule(&self, col: usize, row: usize) -> bool {
        let neighbors = self.count_neighbors(col, row);
        let ind = self.coord_to_ind(col, row);
        let is_cell_alive = self.cells[ind] == GolCells::Alive;
        if (is_cell_alive && (neighbors == 2 || neighbors == 3))
            || (!is_cell_alive && neighbors == 3)
        {
            return true;
        }
        false
    }

    fn count_neighbors(&self, col: usize, row: usize) -> u8 {
        let mut neighbors = 0;
        for off_x in -1i32..=1 {
            for off_y in -1i32..=1 {
                if off_x == 0 && off_y == 0 {
                    continue;
                }
                let curr_col = col as i32 + off_x;
                let curr_row = row as i32 + off_y;
                if self.is_in_bound(curr_col, true)
                    && self.is_in_bound(curr_row, false)
                    && self.is_alive(curr_col as usize, curr_row as usize)
                {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn is_in_bound(&self, num: i32, is_col: bool) -> bool {
        if num > 0 {
            return match is_col {
                true => num < self.x_count as i32,
                false => num < self.y_count as i32,
            };
        }
        false
    }

    fn coord_to_ind(&self, col: usize, row: usize) -> usize {
        (col * self.x_count) + row
    }
}

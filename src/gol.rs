#[derive(Copy, Clone, PartialEq)]
enum GolCells {
    Dead,
    Alive,
}

pub struct Gol {
    cells: Vec<GolCells>,
    width: usize,
    height: usize,
}

impl Gol {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![GolCells::Dead; width * height],
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let mut to_be_alive: Vec<usize> = vec![];
        let mut to_be_dead: Vec<usize> = vec![];
        for col in 0..self.width {
            for row in 0..self.height {
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

    pub fn apply_rule(&self, col: usize, row: usize) -> bool {
        let neighbors = self.count_neighbors(col, row);
        let ind = self.coord_to_ind(col, row);
        let is_cell_alive = self.cells[ind] == GolCells::Alive;
        if is_cell_alive && (neighbors == 2 || neighbors == 3) {
            return true;
        } else if !is_cell_alive && neighbors == 3 {
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
                if self.is_in_bound(curr_col, true) && self.is_in_bound(curr_row, false) {
                    if self.is_alive(curr_col as usize, curr_row as usize) {
                        neighbors += 1;
                    }
                }
            }
        }
        neighbors
    }

    fn is_alive(&self, col: usize, row: usize) -> bool {
        let ind = self.coord_to_ind(col, row);
        self.cells[ind] == GolCells::Alive
    }

    fn is_in_bound(&self, num: i32, is_col: bool) -> bool {
        if num > 0 {
            return match is_col {
                true => num < self.width as i32,
                false => num < self.height as i32,
            };
        }
        false
    }

    pub fn toggle(&mut self, col: usize, row: usize) {
        let ind = self.coord_to_ind(col, row);
        if self.cells[ind] == GolCells::Alive {
            self.cells[ind] = GolCells::Dead;
        } else {
            self.cells[ind] = GolCells::Alive;
        }
    }

    fn coord_to_ind(&self, col: usize, row: usize) -> usize {
        (col * self.width) + (row + 1)
    }
}

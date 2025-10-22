use rand::Rng;

pub struct Conway {
    pub cellules: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Conway {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cellules: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn alive(&self, row: usize, col: usize) -> bool {
        self.cellules[row * self.width + col]
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        let i = row * self.width + col;
        self.cellules[i] = !self.cellules[i];
    }

    pub fn random_mutate(&mut self) {
        let mut rng = rand::rng();
        self.cellules.iter_mut().for_each(|c| *c = rng.random());
    }

    pub fn reset(&mut self) {
        self.cellules.iter_mut().for_each(|c| *c = false);
    }

    pub fn step(&mut self) {
        let mut to_toggle = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let n = self.live_neighbours(row as isize, col as isize);
                if (self.alive(row, col) && (n <= 1 || n > 3)) || (!self.alive(row, col) && n == 3)
                {
                    to_toggle.push((row, col));
                }
            }
        }
        to_toggle
            .iter()
            .for_each(|(row, col)| self.toggle(*row, *col));
    }

    fn live_neighbours(&self, row: isize, col: isize) -> usize {
        (-1..=1)
            .flat_map(|r| (-1..=1).map(move |c| (r, c)))
            .filter(|&(r, c)| (r, c) != (0, 0))
            .filter(|&(r, c)| self.cellules[self.row_col_as_idx(row + r, col + c)])
            .count()
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.height as isize);
        let col = wrap(col, self.width as isize);
        row * self.width + col
    }
}

fn wrap(idx: isize, range: isize) -> usize {
    ((idx % range + range) % range) as usize // because % has sign of dividend
}

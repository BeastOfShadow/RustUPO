use std::vec;
use rand::Rng;

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ' ',
                Self::Wall => '+',
                Self::Food(_) => 'F',
                _ => 'P',
            }
        )
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix.len() {
                write!(f, "{}", self.matrix[i][j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cell {
    Empty,
    Food(i32),
    Poison(i32),
    Wall
}

pub struct Table {
    pub matrix: Vec<Vec<Cell>>
}

impl Table {
    pub fn new(n: usize, mut m: usize, food_val: i32, poison_val: i32) -> Self {
        let mut grid = vec![vec![Cell::Empty; n]; n];

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if i == 0 || j == 0 || grid.len() -1 == i || grid.len() -1 == j {
                    grid[i][j] = Cell::Wall;
                }
            }
        }

        let mut rnd = rand::thread_rng();
        let half_loop = m / 2;

        while 0 != m {
            let rand_r = rnd.gen_range(1..n - 1);
            let rand_c = rnd.gen_range(1..n - 1);

            if grid[rand_r][rand_c] == Cell::Empty {
                if half_loop >= m { 
                    grid[rand_r][rand_c] = Cell::Food(food_val);
                } else {
                    grid[rand_r][rand_c] = Cell::Poison(poison_val);
                }
                m -= 1;
            }
        }

        Self {
            matrix: grid,
        }
    }

    pub fn clear_cell(&mut self, c: usize, r: usize) {
        self.matrix[r][c] = Cell::Empty;
    }
}
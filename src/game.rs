use crate::{
    player::{Direction, Player},
    table::{Cell, Table},
};
use rand::Rng;
impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.table.matrix.len() {
            for j in 0..self.table.matrix.len() {
                if i == self.player.row as usize && j == self.player.col as usize {
                    write!(f, "{}", self.player.cur_dir)?;
                } else {
                    write!(f, "{}", self.table.matrix[i][j])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub struct Configuration {
    table: Table,
    pub player: Player,
}

impl Configuration {
    pub fn new(n: usize, m: usize, food_val: i32, poison_val: i32, player_strength: i32) -> Self {
        let table = Table::new(n, m * 2, food_val, poison_val);

        let mut rnd = rand::thread_rng();

        let mut rand_r = rnd.gen_range(1..n - 1);
        let mut rand_c = rnd.gen_range(1..n - 1);

        while table.matrix[rand_r][rand_c] != Cell::Empty {
            rand_r = rnd.gen_range(1..n - 1);
            rand_c = rnd.gen_range(1..n - 1);
        }

        let player = Player::new(rand_c, rand_r, player_strength);

        Configuration { table, player }
    }

    pub fn interaction(&mut self, c: usize, r: usize) {
        match self.table.matrix[r][c] {
            Cell::Empty => self.player.move_player(c, r),
            Cell::Food(i) => {
                self.player.move_player(c, r);
                self.player.update_strength(i);
                self.table.clear_cell(c, r);
            }
            Cell::Poison(i) => {
                self.player.move_player(c, r);
                self.player.update_strength(i);
                self.table.clear_cell(c, r);
            }
            Cell::Wall => self.player.opposite_direction(),
        }
    }

    pub fn player_action(&mut self) {
        match &self.player.cur_dir {
            Direction::Up => self.interaction(self.player.col, self.player.row - 1),
            Direction::Down => self.interaction(self.player.col, self.player.row + 1),
            Direction::Left => self.interaction(self.player.col - 1, self.player.row),
            _ => self.interaction(self.player.col + 1, self.player.row),
        }
    }

    pub fn play_turn(&mut self) {
        let flip = Player::flip_coin();

        if flip == 1 {
            self.player.cur_dir = Player::change_direction();
            println!("Change direction! Now you are: {}", self.player.cur_dir);
        } 

        self.player_action();
    }
}

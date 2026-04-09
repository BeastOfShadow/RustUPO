use rand::Rng;

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => '↑',
                Self::Down => '↓',
                Self::Left => '←',
                _ => '→',
            }
        )
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cur_dir)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub col: usize,
    pub row: usize,
    pub strength: i32,
    pub cur_dir: Direction,
}

impl Player {
    pub fn new(col: usize, row: usize, strength: i32) -> Self {
        Self {
            col,
            row,
            strength,
            cur_dir: Self::change_direction(),
        }
    }

    pub fn flip_coin() -> i32 {
        let mut rnd = rand::thread_rng();
        let flip = rnd.gen_range(0..2);

        println!("Flipping coin...");
        if flip == 0 {
            println!("Head!");
        } else {
            println!("Tail!");
        }

        flip
    }

    pub fn move_player(&mut self, c: usize, r: usize) {
        self.col = c;
        self.row = r;
    }

    pub fn update_strength(&mut self, val: i32) {
        self.strength += val;
    }

    pub fn opposite_direction(&mut self) {
        match &self.cur_dir {
            Direction::Up => self.cur_dir = Direction::Down,
            Direction::Down => self.cur_dir = Direction::Up,
            Direction::Left => self.cur_dir = Direction::Right,
            _ => self.cur_dir = Direction::Left,
        }
    }

    pub fn change_direction() -> Direction {
        match rand::thread_rng().gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

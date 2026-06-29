use esercizi::game::Configuration;
use esercizi::player::{Direction, Player};
use esercizi::table::{Cell, Table};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_player_logic() {
        let mut player = Player::new(1, 1, 10);

        player.update_strength(5);
        assert_eq!(
            player.strength, 15,
            "Strength does not update correctly with food"
        );

        player.update_strength(-20);
        assert_eq!(
            player.strength, -5,
            "Strength does not update correctly with poison"
        );

        player.cur_dir = Direction::Up;
        player.opposite_direction();
        assert_eq!(
            player.cur_dir,
            Direction::Down,
            "The opposite of Up must be Down"
        );
    }

    #[test]
    fn test_table_logic() {
        let n = 5;
        let mut table = Table::new(n, 2, 5, -10);

        assert_eq!(
            table.matrix[0][0],
            Cell::Wall,
            "The top-left corner is not a wall"
        );
        assert_eq!(
            table.matrix[n - 1][n - 1],
            Cell::Wall,
            "The bottom-right corner is not a wall"
        );
        assert_ne!(
            table.matrix[2][2],
            Cell::Wall,
            "The center must not be a wall"
        );

        table.matrix[2][2] = Cell::Food(5);
        table.clear_cell(2, 2);
        assert_eq!(
            table.matrix[2][2],
            Cell::Empty,
            "The cell was not emptied"
        );
    }

    #[test]
    fn test_game_interaction() {
        let mut game = Configuration::new(5, 2, 5, -5, 10);

        game.player.col = 1;
        game.player.row = 1;
        game.player.cur_dir = Direction::Up;

        game.player_action();

        assert_eq!(game.player.row, 1, "The player passed through the wall!");
        assert_eq!(
            game.player.cur_dir,
            Direction::Down,
            "The player did not bounce off the wall"
        );
    }
}

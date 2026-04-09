use esercizi::game::Configuration;
use esercizi::player::{Direction, Player};
use esercizi::table::{Cell, Table};

#[cfg(test)]
mod tests {
    use super::*;
    // --- TEST DEL GIOCATORE ---
    #[test]
    fn test_player_logic() {
        let mut player = Player::new(1, 1, 10);

        player.update_strength(5);
        assert_eq!(
            player.strength, 15,
            "La forza non si aggiorna correttamente col cibo"
        );

        player.update_strength(-20);
        assert_eq!(
            player.strength, -5,
            "La forza non si aggiorna correttamente col veleno"
        );

        player.cur_dir = Direction::Up;
        player.opposite_direction();
        assert_eq!(
            player.cur_dir,
            Direction::Down,
            "L'opposto di Up deve essere Down"
        );
    }

    // --- TEST DELLA MAPPA ---
    #[test]
    fn test_table_logic() {
        let n = 5;
        let mut table = Table::new(n, 2, 5, -10);

        assert_eq!(
            table.matrix[0][0],
            Cell::Wall,
            "L'angolo in alto a sinistra non è un muro"
        );
        assert_eq!(
            table.matrix[n - 1][n - 1],
            Cell::Wall,
            "L'angolo in basso a destra non è un muro"
        );
        assert_ne!(
            table.matrix[2][2],
            Cell::Wall,
            "Il centro non deve essere un muro"
        );

        // Pulizia della cella
        table.matrix[2][2] = Cell::Food(5);
        table.clear_cell(2, 2);
        assert_eq!(
            table.matrix[2][2],
            Cell::Empty,
            "La cella non si è svuotata"
        );
    }

    // --- TEST DI INTEGRAZIONE (GAME LOOP) ---
    #[test]
    fn test_game_interaction() {
        let mut game = Configuration::new(5, 2, 5, -5, 10);

        game.player.col = 1;
        game.player.row = 1;
        game.player.cur_dir = Direction::Up;

        game.player_action();

        assert_eq!(game.player.row, 1, "Il giocatore ha attraversato il muro!");
        assert_eq!(
            game.player.cur_dir,
            Direction::Down,
            "Il giocatore non è rimbalzato sul muro"
        );
    }
}

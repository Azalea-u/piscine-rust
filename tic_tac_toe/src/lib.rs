pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if has_won('O', table) {
        "player O won".to_string()
    } else if has_won('X', table) {
        "player X won".to_string()
    } else {
        "tie".to_string()
    }
}

fn has_won(player: char, table: [[char; 3]; 3]) -> bool {
    horizontal(player, table) || vertical(player, table) || diagonals(player, table)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in &table {
        if row.iter().all(|&c| c == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if (0..3).all(|row| table[row][col] == player) {
            return true;
        }
    }
    false
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let main_diagonal = (0..3).all(|i| table[i][i] == player);
    let anti_diagonal = (0..3).all(|i| table[i][2 - i] == player);
    main_diagonal || anti_diagonal
}

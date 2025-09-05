pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if has_won('O', &table) {
        "player O won".to_string()
    } else if has_won('X', &table) {
        "player X won".to_string()
    } else {
        "tie".to_string()
    }
}

fn has_won(player: char, table: &[[char; 3]; 3]) -> bool {
    horizontal(player, table) || vertical(player, table) || diagonals(player, table)
}

pub fn horizontal(player: char, table: &[[char; 3]; 3]) -> bool {
    for row in table {
        if (0..3).all(|col| row[col] == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: &[[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if (0..3).all(|row| table[row][col] == player) {
            return true;
        }
    }
    false
}

pub fn diagonals(player: char, table: &[[char; 3]; 3]) -> bool {
    let mut main_diagonal = true;
    let mut anti_diagonal = true;
    for i in 0..3 {
        main_diagonal = main_diagonal && table[i][i] == player;
        anti_diagonal = anti_diagonal && table[i][2 - i] == player;
    }
    main_diagonal || anti_diagonal
}

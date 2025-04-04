pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|row| row.iter().all(|&c| c == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..3).any(|col| (0..3).all(|row| table[row][col] == player))
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let diag1 = (0..3).all(|i| table[i][i] == player);
    let diag2 = (0..3).all(|i| table[i][2 - i] == player);
    diag1 || diag2
}

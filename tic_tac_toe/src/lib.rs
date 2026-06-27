pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('X', table)
    || vertical('X', table)
    || diagonals('X', table){
        "player X won".to_string()

    }else if horizontal('O', table)
    || vertical('O', table)
    || diagonals('O', table){
        "player O won".to_string()
    }else{
        "tie".to_string()
    }
    
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] ==player&& table[2][2]==player)||
    (table[0][2]==player&&table[1][1]==player&&table[2][0]==player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0]==player&&table[0][1]==player&&table[0][2]==player)||
    (table[1][0]==player&&table[1][1]==player&&table[1][2]==player)||
    (table[2][0]==player&&table[2][1]==player&&table[2][2]==player)
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0]==player&&table[1][0]==player&&table[2][0]==player)||
    (table[0][1]==player&&table[1][1]==player&&table[2][1]==player)||
    (table[0][2]==player&&table[1][2]==player&&table[2][2]==player)
}

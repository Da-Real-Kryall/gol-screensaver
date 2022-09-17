
pub(crate) fn print_board(board: &Vec<Vec<usize>>) {
    for row in board {
        for cell in row {
            match cell {
                0 => print!(" "),
                1 => print!("█"),
                _ => print!(" "),
            }
        }
        println!();
    }
}
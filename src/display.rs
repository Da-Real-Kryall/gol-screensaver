use termion::{self, cursor::Goto};

pub(crate) fn print_board(
    board: &Vec<Vec<usize>>,
    old_board: &Vec<Vec<usize>>,
    colours: [usize; 5],
    old_colours: [usize; 5],
    characters: [char; 6],
    old_characters: [char; 6],
    lifetype: usize,
) {
    let mut print_buffer = String::new();
    print_buffer.push_str(termion::cursor::Goto(1, 1).to_string().as_str());
    let mut colour: usize = 1000;
    let mut just_updated: bool = false;
    for _y in 0..board.len() + 2 {
        for _x in 0..board[0].len() + 2 {
            let x = (_x + board[0].len() - 1) % board[0].len();
            let y = (_y + board.len() - 1) % board.len();
            let mut state = board[y][x];
            let mut old_state = old_board[y][x];

            if (_x.abs_diff(board[0].len() + 1) % (board[0].len() + 1) < 1
                || _y.abs_diff(board.len() + 1) % (board.len() + 1) < 1)
                && state != 0
            {
                state = state.min(1) + 4;
                old_state = old_state.min(1) + 4;
            }
            if old_state != state
                || {
                    (colours != old_colours || characters[state] != old_characters[state])
                        && state != 0
                }
                || !(lifetype == 1
                    || lifetype == 2
                    || lifetype == 3
                    || lifetype == 7
                    || lifetype == 9)
            {
                if just_updated == false
                    && (lifetype == 1
                        || lifetype == 2
                        || lifetype == 3
                        || lifetype == 7
                        || lifetype == 9)
                {
                    print_buffer.push_str(&format!("{}", Goto((_x + 1) as u16, (_y + 1) as u16)));
                    just_updated = true;
                }
                if board[y][x] != 0 && colour != colours[state - 1] {
                    colour = colours[board[y][x] - 1];
                    print_buffer.push_str(&format!("\x1b[1;38;5;{}m", colour));
                }
                print_buffer.push(characters[state]);
            } else {
                just_updated = false;
            }
        }
    }
    print!("{}", print_buffer);
}

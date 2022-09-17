use termion;

pub(crate) fn print_board(board: &Vec<Vec<usize>>, colours: [usize; 6]) {
    let mut print_buffer = String::new();
    //let backdrop = COLOUR_REF[*lifetype][1];
    print_buffer.push_str(&format!("\x1b[38;5;{}m{}", colours[0], termion::cursor::Goto(1, 1)));
    //print_buffer.push_str(&format!("\x1b[0m{}", termion::cursor::Goto(1, 1)));

    let mut bg_colour = colours[0];
    let mut fg_colour = colours[1];

    for y in 0..board.len()/2 {
        for x in 0..board[0].len() {
            if bg_colour != colours[board[y*2+1][x]] {
                bg_colour = colours[board[y*2+1][x]];
                print_buffer.push_str(&format!("\x1b[48;5;{}m", bg_colour));
            }
            if fg_colour != colours[board[y*2][x]] {
                fg_colour = colours[board[y*2][x]];
                print_buffer.push_str(&format!("\x1b[38;5;{}m", fg_colour));
            }
            if fg_colour != bg_colour {
                print_buffer.push_str("▀");
            } else {
                print_buffer.push_str("█");
            }
            




            //let bg_colour = COLOUR_REF[*lifetype][board[y*2][x]];
            //let fg_colour = COLOUR_REF[*lifetype][board[y*2+1][x]];
            //match (fg_colour == backdrop, bg_colour == backdrop) {
            //    _ => {}
            //}
            //print_buffer.push_str(&format!("\x1b[48;5;{};38;5;{}m▀", fg_colour, bg_colour));
            //if fg_colour == print_buffer.push_str("▀");
            //print_buffer.push_str(&format!("\x1b["))
            //match (board[y*2][x] == 1, board[y*2+1][x] == 1) {
            //    (false, false) => print_buffer.push_str(" "),
            //    (false, true) => print_buffer.push_str("▄"),
            //    (true, false) => print_buffer.push_str("▀"),
            //    (true, true) => print_buffer.push_str("█"),
            //}
        }
    }
    print!("{}", print_buffer);
}
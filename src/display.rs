use termion::{self, cursor::Goto};

pub(crate) fn print_board(board: &Vec<Vec<usize>>, old_board: &Vec<Vec<usize>>, colours: [usize; 5], old_colours: [usize; 5], characters: [char; 6], old_characters: [char; 6]) {
    let mut print_buffer = String::new();
    let mut colour: usize = 1000;
    let mut just_updated: bool = false;
    for _y in 0..board.len()+2 {
        for _x in 0..board[0].len()+2 {
            let x = (_x + board[0].len() - 1) % board[0].len();
            let y = (_y + board.len() - 1) % board.len();
            let mut state = board[y][x];
            let mut old_state = old_board[y][x];
            if (_x.abs_diff(board[0].len()+1)%(board[0].len()+1) < 1 || _y.abs_diff(board.len()+1)%(board.len()+1) < 1) && state != 0 {
                state = state.min(2) + 3;
                old_state = old_state.min(2) + 3;
            }
            if old_state != state || {
                (colours != old_colours || characters[state] != old_characters[state]) && state != 0
            }
            {
                if just_updated == false {
                    print_buffer.push_str(&format!("{}", Goto((_x+1) as u16, (_y+1) as u16)));
                    just_updated = true;
                }
                if board[y][x] != 0 && colour != colours[state-1] {
                    colour = colours[state-1];
                    print_buffer.push_str(&format!("\x1b[1;38;5;{}m", colour));
                }
                print_buffer.push(characters[state]);
            } else {
                just_updated = false;
            }
        }
    }
    print!("{}", print_buffer); 
    /*let mut print_buffer = String::new();
    //let backdrop = COLOUR_REF[*lifetype][1];
    print_buffer.push_str(&format!("\x1b[38;5;{}m{}", colours[0], termion::cursor::Goto(1, 1)));
    //print_buffer.push_str(&format!("\x1b[0m{}", termion::cursor::Goto(1, 1)));
    
    //let mut bg_colour = 1000;//;colours[0];
    let mut colour = 1000;//colours[1];
    
    for y in 0..board.len() {
        for x in 0..board[0].len() {

            if board[y][x] != old_board[y][x] {
                if board[y][x.max(1)-1] == old_board[y][x.max(1)-1] {
                    print_buffer.push_str(Goto(x as u16, y as u16).to_string().as_str());
                }


                if board[y][x] != 0 && colour != colours[board[y][x]-1] {
                    colour = colours[board[y][x]-1];
                    print_buffer.push_str(&format!("\x1b[38;5;{}m", colour));
                }
                print_buffer.push(characters[board[y][x]]);
            }
            //if bg_colour != colours[board[y*2+1][x]] {
            //    bg_colour = colours[board[y*2+1][x]];
            //    print_buffer.push_str(&format!("\x1b[48;5;{}m", bg_colour));
            //}
            //if fg_colour != colours[board[y*2][x]] {
            //    fg_colour = colours[board[y*2][x]];
            //    print_buffer.push_str(&format!("\x1b[38;5;{}m", fg_colour));
            //}
            //if fg_colour != bg_colour {
            //    print_buffer.push_str("▀");
            //} else {
            //    if bg_colour == colours[0] {
            //        print_buffer.push_str(" ");
            //    } else {//full block 
            //        print_buffer.push_str("█");
            //    }
            //}
        }
    }
    print!("{}", print_buffer);
    */
}
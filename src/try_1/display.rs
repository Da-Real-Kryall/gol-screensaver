use termion;

pub fn return_board(size: &(u16, u16), board: &Vec<Vec<usize>>, hardset: usize) {
    let mut board_string = String::new();
    for y in 0..size.1 {
        for x in 0..size.0 {
            board_string.push_str(match board[y as usize][x as usize] {
                0 => " ",
                1 => "#",
                2 => "$",
                3 => "%",
                4 => "*",
                5 => "~",
                6 => ".",
                _ => " ",
            });
        }
    }
    println!("{}{}", termion::cursor::Goto(1, 1), board_string);
}
//#include <stdio.h>
//
//void return_board(int width, int height, int boardarray[width][height], int typeref[width+2][height+2], int hardset) {
//
//    char buffer[(width+2) * (height+2)];
//    char charref[7][4] = {
//        {' ', ' ', ' ', ' '},
//        {'#', '-', '|', '*'},
//        {'$', '-', '|', '*'},
//        {'%', '-', '|', '*'},
//        {'*', '-', '|', '*'},
//        {'~', '-', '|', '*'},
//        {'.', '-', '|', '*'}
//    };
//
//    for (int y = 0; y < (height+2); y++) {
//        for (int x = 0; x < (width+2); x++) {
//            buffer[x+(y*(width+2))] = (hardset == -1) ? charref[boardarray[(x+width-1) % width][(y+height-1) % height]][typeref[x][y]] : charref[hardset*(boardarray[(x+width-1) % width][(y+height-1) % height] != 0)][typeref[x][y]];
//        }
//    }
//    buffer[(width+2)*(height+2)] = '\0';
//    printf("%s", buffer);
//    fflush(stdout);
//}
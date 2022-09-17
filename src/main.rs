use std::thread;
use std::time::Duration;
use rand::Rng;

use termion;

//ill move these to constants.rs later
const MS_DELAY: u32 = 60000; //73000
const HISTORY_LENGTH: usize = 20;

fn main() {

    thread::sleep(Duration::from_secs(2));

    //get random generator
    let mut rng = rand::thread_rng();

    //width is size.0 - 2
    //height is size.1 - 2
    let size: (u16, u16) = //{
        //let (w, h) = 
        termion::terminal_size().unwrap();
        //(w - 2, h - 2)
    //};

    //dont need typeref just yet, i'll start without border

    let mut history_offset: usize = 0;

    let mut lifetype_history: [usize; HISTORY_LENGTH] = [0; HISTORY_LENGTH];
    let mut sum_history: [u32; HISTORY_LENGTH] = [0; HISTORY_LENGTH];
    let mut board_history: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; size.0 as usize]; size.1 as usize]; HISTORY_LENGTH];

    let mut limited_life_timer: usize = 0;

    print!("\x1b[?25l"); //hide cursor
    
    loop {
        return_board(&size, &board_history[history_offset], -1);
        
        limited_life_timer -= 1;
    
        if update_board(&size, &history_offset, &lifetype_history, &sum_history, &board_history, 
            evaluate_history(&size, &limited_life_timer, &history_offset, &sum_history, &lifetype_history, &board_history)
        ) {
            //if update_board returns 1 the board has been refilled due to it being completely empty
            for i in 0..5 {
                thread::sleep(Duration::from_millis(MS_DELAY as u64);
                return_board(&size, &board_history[history_offset], 6-i);
            }
        }
        thread::sleep(Duration::from_millis(MS_DELAY as u64));
    }
}
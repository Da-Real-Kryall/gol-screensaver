/*
i have a loop that continuously updates the board
in the board history i keep track of cell states and lifetypes

here are my checks for refilling the board:
    if the board is empty

then, here are my checks for changing the lifetype:
    it has been a while since the last change
    the board is identical to one of the last 16 boards, lifetype doesn't matter when comparing

after these changes happen, the board will still be evaluated and updated.
*/
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

pub(crate) mod consts;
pub(crate) mod display;
pub(crate) mod iterate;
use crate::consts::*;
use crate::display::*;
use crate::iterate::*;

/* Functions to make:
    check_board_history
    refill_board
    update_board
    print_board
*/

fn main() {
    thread::sleep(Duration::from_millis(2000));
    let mut rng = rand::thread_rng();
    let size: (u16, u16) = {
        let mut s = termion::terminal_size().unwrap();
        s.0 -= 2;
        s.1 -= 2;
        s
    };
    for _ in 0..size.0 {
        print!("\n");
    }

    print!("{}", termion::cursor::Hide);

    //show cursor (ansi sequence)
    //print!("\x1b[?25h");

    //wait 2 seconds

    let mut state_history: VecDeque<Vec<Vec<usize>>> =
        VecDeque::from(vec![
            vec![vec![0; size.0 as usize]; size.1 as usize];
            HISTORY_LENGTH
        ]);
    let mut type_history: VecDeque<usize> = VecDeque::from(vec![0; HISTORY_LENGTH]);
    let mut limited_life_timer: usize = 0;

    let mut colour_palette: [usize; 6] = [0; 6];
    let mut old_colour_palette: [usize; 6] = [0; 6];

    let mut char_palette: [usize; 6] = [0; 6];
    let mut old_char_palette: [usize; 6] = [0; 6];

    let mut current: Instant;
    loop {
        current = Instant::now();

        let mut new_board = state_history[0].clone();
        let mut new_type: usize = type_history[0];

        match check_board_history(&state_history, &limited_life_timer) {
            //0: board is empty, refill it and change lifetype
            //1: board is identical to one of the last 16 boards, change lifetype
            //2: board has been the same for a while, change lifetype
            1 | 2 => {
                limited_life_timer = 150 + rng.gen_range(0, 250); //also reset limited_life_timer

                //new_type = rng.gen_range(0, LIFE_REF.len());
                //check if the next board iteration will be empty with this type, and if so, change it.
                //keep changing and checking until the next board will not be empty
                //shuffled range of 0 to life_ref.len() - 1
                let mut shuffled_range: Vec<usize> = (0..LIFE_REF.len()).collect();
                shuffled_range.shuffle(&mut rng);

                for lifetype in shuffled_range {
                    //new_type = lifetype;
                    //break;
                    let empty = update_board(&new_board, &lifetype, &false)
                        .iter()
                        .flatten()
                        .all(|&x| x != 1);
                    if !empty {
                        new_type = rng.gen_range(0, LIFE_REF.len());
                        break;
                    }
                }
            }
            0 => {
                limited_life_timer = 150 + rng.gen_range(0, 250); //also reset limited_life_timer
                new_type = rng.gen_range(0, LIFE_REF.len());

                //previous board is empty, refill after doing all the other stuff first
                new_board = refill_board(&new_board, &type_history[0]); //see if i can remove this clone later
                colour_palette = [0; 6];
                char_palette = [0; 6];
            }
            _ => {}
        }
        //update the board
        new_board = update_board(
            &new_board,
            &type_history[0],
            &(new_type != type_history[0]), // && new_type - 3 < 3),
        ); //see if i can remove this clone later

        //update the history and limited life timer
        state_history.pop_back();
        state_history.push_front(new_board);

        type_history.pop_back();
        type_history.push_front(new_type);

        limited_life_timer -= 1;

        if colour_palette != COLOUR_REF[type_history[0]] {
            //old_colour_palette = colour_palette.clone();
            //make colour palette closer to the current lifetype's palette
            for i in 0..colour_palette.len() {
                let mut colour: [u32; 3] = PALETTE[colour_palette[i]];
                let target: [u32; 3] = PALETTE[COLOUR_REF[type_history[0]][i]];
                for j in 0..colour.len() {
                    let tmp = colour[j] as i32 - target[j] as i32;
                    if tmp > 0 {
                        colour[j] -= 50.min(tmp as u32);
                    } else if tmp < 0 {
                        colour[j] += 50.min(-tmp as u32);
                    }
                }
                //convert colour to closest palette colour
                let mut closest: usize = 0;
                let mut closest_dist: u32 = 1000;
                for j in 0..PALETTE.len() {
                    let mut dist: u32 = 0;
                    for k in 0..colour.len() {
                        dist += (colour[k] as i32 - PALETTE[j][k] as i32).abs() as u32;
                    }
                    if dist < closest_dist {
                        closest = j;
                        closest_dist = dist;
                    }
                }
                old_colour_palette[i] = colour_palette[i];
                colour_palette[i] = closest; //PALETTE.iter().position(|&x| x == colour).unwrap();
            }
        }

        if char_palette != CHAR_REF[type_history[0]] {
            //make the char palette closer to the current lifetype's palette, preferrably one character at a time
            for i in 0..char_palette.len() {
                if char_palette[i] == CHAR_REF[type_history[0]][i] {
                    continue;
                }
                let mut current: (usize, usize) = (char_palette[i] % 5, char_palette[i] / 5);
                let target: (usize, usize) = (
                    CHAR_REF[type_history[0]][i] % 5,
                    CHAR_REF[type_history[0]][i] / 5,
                );
                let dist_x: usize = current.0.abs_diff(target.0);
                let dist_y: usize = current.1.abs_diff(target.1);
                if dist_x > dist_y {
                    if current.0 > target.0 {
                        current.0 -= 1;
                    } else {
                        current.0 += 1;
                    }
                } else {
                    if current.1 > target.1 {
                        current.1 -= 1;
                    } else {
                        current.1 += 1;
                    }
                }
                old_char_palette[i] = char_palette[i];
                char_palette[i] = current.0 + current.1 * 5;
            }
        }

        //print the board

        print_board(
            &state_history[0],
            &state_history[1],
            colour_palette,
            old_colour_palette,
            char_palette,     //CHAR_PALETTE[type_history[0]],
            old_char_palette, //CHAR_PALETTE[type_history[1]],
        );
        let duration = current.elapsed();
        //print duration
        //if duration > Duration::from_millis(1) {
        //    println!("{}.{:03}", duration.as_secs(), duration.subsec_millis());
        //}
        thread::sleep(Duration::from_millis(
            DELAY_MS - (duration.as_millis() as u64).min(DELAY_MS),
        ));
    }
}

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
use std::collections::VecDeque;
use rand::Rng;
use std::thread;
use std::time::Duration;

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
    let mut rng = rand::thread_rng();
    let size: (u16, u16) = {
        let mut s = termion::terminal_size().unwrap();
        s.1 *= 2;
        s
    };
    let mut state_history: VecDeque<Vec<Vec<usize>>> = VecDeque::from(vec![vec![vec![0; size.0 as usize]; size.1 as usize]; HISTORY_LENGTH]);
    let mut type_history: VecDeque<usize> = VecDeque::from(vec![0; HISTORY_LENGTH]);
    let mut limited_life_timer: usize = 0;
    let mut colour_palette: [usize; 6] = COLOUR_REF[0];

    loop {
        thread::sleep(Duration::from_millis(DELAY_MS));
        //let new_board = board_history[0].clone();
        //check if board should be changed, and how it should be changed
        let mut new_board = state_history[0].clone();
        let mut new_type = type_history[0];

        match check_board_history(&state_history, &limited_life_timer) { //&type_history,
            //0: board is empty, refill it and change lifetype
            //1: board is identical to one of the last 16 boards, change lifetype
            //2: board has been the same for a while, change lifetype
            1 | 2 => {
                limited_life_timer = 150+rng.gen_range(0, 250); //also reset limited_life_timer
                
                new_type = rng.gen_range(0, LIFE_REF.len());
            },
            0 => {
                limited_life_timer = 150+rng.gen_range(0, 250); //also reset limited_life_timer
                new_type = rng.gen_range(0, LIFE_REF.len());

                //board is empty, refill after doing all the other stuff first
                new_board = refill_board(&new_board, &type_history[0].clone()); //see if i can remove this clone later
            },
            _ => {
            },
        }
        //update the board
        new_board = update_board(&new_board, &type_history[0].clone()); //see if i can remove this clone later
        
        //update the history and limited life timer
        state_history.pop_back();
        state_history.push_front(new_board);

        type_history.pop_back();
        type_history.push_front(new_type);

        limited_life_timer -= 1;

        if colour_palette != COLOUR_REF[type_history[0]] {
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
                colour_palette[i] = closest;//PALETTE.iter().position(|&x| x == colour).unwrap();
            }
        }

        //print the board
        print_board(&state_history[0], colour_palette);
    }

}
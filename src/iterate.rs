
use std::collections::VecDeque;
use rand::Rng;
use crate::consts::{
    INIT_CHANCE_REF,
    LIFE_REF,
};


//holy frick, copilot wrote all of this
pub(crate) fn check_board_history(state_history: &VecDeque<Vec<Vec<usize>>>, limited_life_timer: &usize) -> usize {//, type_history: &VecDeque<usize>
    //check if board is empty
    let mut empty = true;
    for row in &state_history[0] {
        for cell in row {
            if *cell != 0 {
                empty = false;
            }
        }
    }
    if empty {
        return 0;
    }
    //check if board is identical to one of the last 16 boards
    let mut identical = false;
    for i in 1..state_history.len() {
        if state_history[0] == state_history[i] {
            identical = true;
        }
    }
    if identical {
        return 1;
    }
    //check if board has been the same for a while
    if *limited_life_timer == 0 {
        return 2;
    }
    //board is not empty, not identical to one of the last 16 boards, and has not been the same for a while
    return 3;
}

pub(crate) fn refill_board(new_board: &Vec<Vec<usize>>, new_type: &usize) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut new_board = new_board.clone();
    for row in &mut new_board {
        for cell in row {
            if rng.gen_range(0, INIT_CHANCE_REF[*new_type]) == 0 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    };
    new_board
}
//when x and y are out of bounds they loop back
pub(crate) fn update_board(old_board: &Vec<Vec<usize>>, new_type: &usize) -> Vec<Vec<usize>> {
    let mut new_board = old_board.clone();
    for y in 0..old_board.len() {
        for x in 0..old_board[y].len() {
            let mut neighbors = 0;
            for i in -1..2 {
                for j in -1..2 {
                    if old_board[(y as i32 + i) as usize % old_board.len()][(x as i32 + j) as usize % old_board[y].len()] == 1 {
                        neighbors += 1;
                    }
                }
            }
            if old_board[y][x] == 1 {
                neighbors -= 1;
            }
            new_board[y][x] = LIFE_REF[*new_type][old_board[y][x]][neighbors];
        }
    }
    new_board
}

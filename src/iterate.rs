use crate::consts::{
    HISTORY_EMPTY_CHECK, HISTORY_LENGTH, INIT_CHANCE_REF, LIFE_REF, MAX_FUTURE_CHECK,
};
use rand::Rng;
use std::collections::VecDeque;

//0: board is empty, refill it and change lifetype
//1: board is identical to one of the last few boards, change lifetype
//2: board has been the same for a while, change lifetype
//3: board is GOING to be empty, change lifetype to one that will not be empty

//holy frick, copilot wrote all of this (well, not all of it anymore since i've edited it)
pub(crate) fn check_board_history(
    state_history: &VecDeque<Vec<Vec<usize>>>,
    type_history: &VecDeque<usize>,
    limited_life_timer: &usize,
) -> usize {
    //, type_history: &VecDeque<usize>

    //check if board has been empty for however many rounds
    let empty = (&state_history[HISTORY_EMPTY_CHECK.min(HISTORY_LENGTH)])
        .iter()
        .flatten()
        .all(|&x| x != 1)
        && (&state_history[0]).iter().flatten().all(|&x| x != 1);
    if empty {
        return 0;
    }

    //check if board is GOING to be empty with the current lifetype
    let new_board = state_history[0].clone();
    let lifetype = type_history[0];
    let empty = {
        let mut b = update_board(&new_board, &lifetype, &false);
        for _ in 0..MAX_FUTURE_CHECK {
            b = update_board(&b, &lifetype, &false);
        }
        b
    }
    .iter()
    .flatten()
    .all(|&x| x != 1);

    if empty {
        return 3;
    }

    //check if board is identical to one of the last HISTORY_MIN_CHECK.min(HISTORY_LENGTH)'th HISTORY_LENGTH boards
    let mut identical = false;
    for i in 2..state_history.len() {
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

    //board is not empty, not identical to one of the last 16 boards, and has not been the same for a while, AND isn't going to empty in the next 10 rounds
    return 4;
}
pub(crate) fn refill_board(new_board: &Vec<Vec<usize>>, new_type: &usize) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut new_board = new_board.clone();
    for row in &mut new_board {
        for cell in row {
            if rng.gen_range(0..INIT_CHANCE_REF[*new_type]) == 0 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    }
    new_board
}
//when x and y are out of bounds they loop back
pub(crate) fn update_board(
    old_board: &Vec<Vec<usize>>,
    new_type: &usize,
    soften: &bool,
) -> Vec<Vec<usize>> {
    //let mut sum: usize = 0;
    let mut new_board = old_board.clone();
    for y in 0..old_board.len() {
        for x in 0..old_board[y].len() {
            let mut neighbors = 0;
            for i in -1..2 {
                for j in -1..2 {
                    if old_board[(y as i32 + i + old_board.len() as i32) as usize % old_board.len()]
                        [(x as i32 + j + old_board[0].len() as i32) as usize % old_board[0].len()]
                        == 1
                    {
                        neighbors += 1;
                    }
                }
            }
            if old_board[y][x] == 1 {
                neighbors -= 1;
            }
            //if new type is in [3, 4, 5]
            new_board[y][x] = LIFE_REF[*new_type][old_board[y][x]][neighbors];
            //sum += (new_board[y][x] == 0) as usize;
            if *soften && old_board[y][x] != 0 && new_board[y][x] == 0 {
                new_board[y][x] = 2;
            }
        }
    }
    //if *new_type - 3 < 3 && sum > old_board.len() * old_board[0].len() / 3 {
    //
    //}

    new_board
}

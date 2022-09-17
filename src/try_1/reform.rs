use rand::Rng;
use crate::constants::*;

pub fn standard_deviation(list: &Vec<usize>) -> f64 {
    let length = list.len();

    let mut sum: usize = 0;
    for i in 0..length {
        sum += list[i];
    }
    let mean: f64 = sum as f64 / length as f64;
    let mut sum: f64 = 0.0;
    for i in 0..length {
        sum += (list[i] as f64 - mean).powi(2);
    }
    (sum / (length-1) as f64).sqrt()
}

const larger_types: [usize; 5] = [ //maze, like star wars, coagulations, living on the edge?
    1, 3, 5, 9, 2
];
const smaller_types: [usize; 9] = [ //
    0, 1, 4, 6, 7, 8, 9, 10, 11
];
const large_enders: [usize; 3] = [ //types that have large swathes of the board die usually when the type changes.
    //day night, coagulations, maze
    3, 4, 5
];

const init_chance_ref: [usize; 12] = [
    8,
    4,
    5,
    12,
    2,
    32,
    7,
    2,
    6,
    4,
    2,
    2
];

pub fn fill(size: &(u16, u16), board: &Vec<Vec<usize>>, lifetype: &usize) -> Vec<Vec<usize>> {

    let mut new_board: Vec<Vec<usize>> = vec![vec![0; size.0 as usize]; size.1 as usize];
    //init rand
    let mut rng = rand::thread_rng();

    for y in 0..size.1 as usize {
        for x in 0..size.0 as usize {
            if board[y][x] == *lifetype {
                //if a random number mod init_chance_ref[lifetype] is 1, then the cell is alive
                new_board[y][x] = if rng.gen_range(0, init_chance_ref[*lifetype]) == 1 {*lifetype} else {0};
            }
        }
    };

    new_board

    //for (int y = 0; y < height; y++) {
    //    for (int x = 0; x < width; x++) {
    //        board[x][y] = (rand() % init_chance_ref[lifetype] == 1);
    //    }
    //}
}

pub fn reasign_lifetype(size: &(u16, u16), history_offset: &usize, sum_history: &Vec<u32>, lifetype_history: &Vec<usize>, board_history: &Vec<Vec<Vec<usize>>>) -> Vec<usize> {
    let mut lifetype_history = lifetype_history.clone();

    let mut rng = rand::thread_rng();

    //if less than 8% of the board is filled
    let mut sum_mean: f64 = 0.0;
    for i in 0..HISTORY_LENGTH {
        sum_mean += sum_history[i] as f64;
    }
    sum_mean /= (HISTORY_LENGTH as u16*size.0*size.1/100) as f64;

    if sum_mean < 8.0 {
        lifetype_history[*history_offset] = larger_types[rng.gen_range(0, larger_types.len())];
    } else if sum_mean > 60.0 {
        lifetype_history[*history_offset] = smaller_types[rng.gen_range(0, smaller_types.len())];
    } else {
        let new_lifetype = rng.gen_range(0, 12);
        let old_lifetype = lifetype_history[*history_offset];
        lifetype_history[*history_offset] = new_lifetype + (new_lifetype >= old_lifetype) as usize;
    }

    for i in 0..large_enders.len() {
        if lifetype_history[(history_offset+1)%HISTORY_LENGTH] == large_enders[i] {
            return lifetype_history;
        }
    }
    return vec![0; 0];
}

pub fn evaluate_history(size: &(u16, u16), limited_life_timer: &usize, history_offset: &usize, sum_history: &Vec<u32>, lifetype_history: &Vec<usize>, board_history: Vec<Vec<Vec<usize>>>) -> usize {
    let mut limited_life_timer = limited_life_timer.clone();
    let mut rng = rand::thread_rng();
    let new_lim_life: usize = 150+rng.gen_range(0, 250);
    // could probably compress this a bit more; some lines are duplicates
    // but eh it works as is, not a big deal currently
    if sum_history[*history_offset] == 0 {
        limited_life_timer = new_lim_life;
        reasign_lifetype(&size, &history_offset, &sum_history, &lifetype_history, &board_history);
        fill(&size, &board_history[*history_offset], &lifetype_history[*history_offset]);
        return 2;
    }
    
    for i in 0..HISTORY_LENGTH {
        if board_history[*history_offset] == board_history[(history_offset+i)%HISTORY_LENGTH] && lifetype_history[history_offset] == lifetype_history[(history_offset+i)%HISTORY_LENGTH] {
            limited_life_timer = new_lim_life;
            return reasign_lifetype(&size, &history_offset, &sum_history, &lifetype_history, &board_history);
        }
    }
    if (limited_life_timer <= 0 || (standard_deviation(&sum_history) < 15 && rng.gen_range(0,80) == 1)) {
        limited_life_timer = new_lim_life;
        return reasign_lifetype(&size, &history_offset, &sum_history, &lifetype_history, &board_history);
    }

    return 0;
}

/*

//stuff related to keeping the simulation exciting
#include <math.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <unistd.h>

#include "constants.h"

float standard_deviation(int length, int array[]) {
    float mean = 0;
    for (int i = 0; i < length; i++) {
        mean += array[i];
    }

    mean /= length;

    float deviation = 0;
    for (int i = 0; i < length; i++) {
        deviation += pow(array[i]-mean, 2);
    }
    return sqrt(deviation/(length-1));
}


int larger_types[] = { //maze, like star wars, coagulations, living on the edge?
    1, 3, 5, 9, 2
};
int smaller_types[] = { //
    0, 1, 4, 6, 7, 8, 9, 10, 11
};
int large_enders[] = { //types that have large swathes of the board die usually when the type changes.
    //day night, coagulations, maze
    3, 4, 5
};

int init_chance_ref[12] = {
    8,
    4,
    5,
    12,
    2,
    32,
    7,
    2,
    6,
    4,
    2,
    2
};


void fill(int width, int height, int board[][height], int lifetype) {

    srand(time(NULL));

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            board[x][y] = (rand() % init_chance_ref[lifetype] == 1);
        }
    }
}

int reasign_lifetype(int width, int height, int * history_offset, int sum_history[], int lifetype_history[], int board_history[][width][height]) {
    srand(time(NULL));
    //if less than 8% of the board is filled
    float sum_mean = 0;
    for (int i = 0; i < HISTORY_LENGTH; i++) {
        sum_mean += sum_history[i];
    }
    sum_mean /= HISTORY_LENGTH*width*height/100;

    if (sum_mean < 8) { 
        lifetype_history[*history_offset] = rand() % sizeof(larger_types)/sizeof(int);
    } else if (sum_mean > 60) {
        lifetype_history[*history_offset] = rand() % sizeof(smaller_types)/sizeof(int);
    } else {
        int new_lifetype = rand() % 11;
        int old_lifetype = lifetype_history[*history_offset];
        lifetype_history[*history_offset] = new_lifetype + (new_lifetype >= old_lifetype);
    }
    for (int i = 0; i < sizeof(large_enders)/sizeof(int); i++) {
        if (lifetype_history[(*history_offset+1)%HISTORY_LENGTH] == large_enders[i]) {
            return 1;
        }
    }
    return 0;
};

int evaluate_history(int width, int height, int * limited_life_timer, int * history_offset, int sum_history[], int lifetype_history[], int board_history[][width][height]) {
    
    // could probably compress this a bit more; some lines are duplicates
    // but eh it works as is, not a big deal currently
    if (sum_history[*history_offset] == 0) {
        *limited_life_timer = RNDLIFE;
        reasign_lifetype(width, height, history_offset, sum_history, lifetype_history, board_history);
        fill(width, height, board_history[*history_offset], lifetype_history[*history_offset]);
        return 2;
    }

    for (int i = 1; i < HISTORY_LENGTH; i++) {
        if (memcmp(board_history[*history_offset], board_history[(*history_offset+i)%HISTORY_LENGTH], width*height*sizeof(int)) == 0 && lifetype_history[*history_offset] == lifetype_history[(*history_offset+i)%HISTORY_LENGTH]) {
            *limited_life_timer = RNDLIFE;
            return reasign_lifetype(width, height, history_offset, sum_history, lifetype_history, board_history);
        }
    }
    if (*limited_life_timer <= 0 || (standard_deviation(HISTORY_LENGTH, sum_history) < 15 && rand() % 80 == 1)) {
        *limited_life_timer = RNDLIFE;
        return reasign_lifetype(width, height, history_offset, sum_history, lifetype_history, board_history);
    }

    return 0;
};*/
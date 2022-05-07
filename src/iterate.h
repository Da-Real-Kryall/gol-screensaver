#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

int life_ref[12][7][9] = { // new state for a cell depending on various factors
    { //game of life
        {0, 0, 0, 1, 0, 0, 0, 0, 0},
        {0, 0, 1, 1, 0, 0, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //like star wars
        {0, 0, 1, 0, 0, 0, 0, 1, 1},
        {2, 2, 2, 1, 1, 1, 1, 2, 2},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //walled cities
        {0, 0, 0, 0, 1, 1, 1, 1, 1},
        {0, 0, 1, 1, 1, 1, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //coagulations
        {0, 0, 0, 1, 0, 0, 0, 1, 1},
        {0, 0, 1, 1, 0, 1, 1, 1, 1},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //day night
        {0, 0, 0, 1, 0, 0, 1, 1, 1},
        {0, 0, 0, 1, 1, 0, 1, 1, 1},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //maze
        {0, 0, 0, 1, 0, 0, 0, 0, 0},
        {0, 1, 1, 1, 1, 1, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //2x2
        {0, 0, 0, 1, 0, 0, 1, 0, 0},
        {0, 1, 1, 0, 0, 1, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //amoeba with diff die time
        {0, 0, 0, 1, 0, 1, 0, 1, 0},
        {4, 4, 1, 1, 4, 1, 1, 4, 1},
        {3, 3, 3, 3, 3, 3, 3, 3, 3}, // done
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //frogs
        {0, 0, 0, 1, 1, 0, 0, 0, 0},
        {3, 1, 1, 3, 3, 3, 3, 3, 3},
        {4, 4, 4, 4, 4, 4, 4, 4, 4},
        {6, 6, 6, 6, 6, 6, 6, 6, 6}, //done
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //living on the edge
        {0, 0, 0, 1, 0, 0, 0, 1, 0},
        {4, 4, 4, 1, 1, 1, 4, 4, 1},
        {3, 3, 3, 3, 3, 3, 3, 3, 3},
        {4, 4, 4, 4, 4, 4, 4, 4, 4}, //done
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //high life
        {0, 0, 0, 1, 0, 0, 1, 0, 0},
        {0, 0, 1, 1, 0, 0, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3},
        {4, 4, 4, 4, 4, 4, 4, 4, 4}, //done
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
    { //move
        {0, 0, 0, 1, 0, 0, 1, 0, 1},
        {0, 0, 1, 0, 1, 1, 0, 0, 0},
        {3, 3, 3, 3, 3, 3, 3, 3, 3},
        {4, 4, 4, 4, 4, 4, 4, 4, 4}, //done
        {5, 5, 5, 5, 5, 5, 5, 5, 5},
        {6, 6, 6, 6, 6, 6, 6, 6, 6},
        {0, 0, 0, 0, 0, 0, 0, 0, 0}
    },
};

int deltas[8][2] = {
    {-1, -1},
    {-1, 0},
    {-1, 1},
    {0, -1},
    {0, 1},
    {1, -1},
    {1, 0},
    {1, 1}
};

extern const int HISTORY_LENGTH;

//final arg deternines wether delayed cell death (1 to 6, 5, 4, 3, etc) is forced or determined by tables
int update_board(int width, int height, int * history_offset, int lifetype_history[], int sum_history[], int board_history[][width][height], int force_delay) {
    
    int swap_board[width][height];
    int sum = 0;


    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            int neighbours = 0;
            for (int i = 0; i < 8; i++) {
                if (board_history[*history_offset][(x+deltas[i][0]+width)%width][(y+deltas[i][1]+height)%height] == 1) {
                    neighbours++;
                }
            }
            swap_board[x][y] = life_ref[lifetype_history[*history_offset]][board_history[*history_offset][x][y]][neighbours];
            
            // i might try turn this into a single ternary operator in the above line
            if (force_delay == 1 && swap_board[x][y] == 0 && board_history[*history_offset][x][y] == 1) {
                swap_board[x][y] = 2;
            }
            sum += (swap_board[x][y] == 1);// != board_history[*history_offset][x][y]
        }
    }
    *history_offset = (*history_offset + HISTORY_LENGTH - 1) % HISTORY_LENGTH;
    sum_history[*history_offset] = sum;
    lifetype_history[*history_offset] = lifetype_history[(*history_offset+1)%HISTORY_LENGTH];
    memcpy(board_history[*history_offset], swap_board, sizeof(int) * width * height);
    return force_delay == 2;
}

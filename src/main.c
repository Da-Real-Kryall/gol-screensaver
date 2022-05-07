#include <stdio.h> //printf
#include <stdlib.h> // rand() functions
#include <time.h> // time() functions
#include <unistd.h> // sleep()
#include <string.h> // for mem functions
#include <sys/ioctl.h> // ioctl for terminal size

#include "iterate.h" //functions related to iterating the board
#include "display.h" //functions for printing to output
#include "reform.h" //functions for reversing entropy

const int HISTORY_LENGTH = 20; //probably breaks if its less than 5
const int MS_DELAY = 70000;//73000;

int main() {

    srand(time(NULL));

    struct winsize w;
    ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);
    int width = w.ws_col-2;        //make smaller five times
    int height = w.ws_row-2;


    int typeref[width+2][height+2];
    // {'#', '-', '|', '*'}
    for (int y = 0; y < (height+2); y++) {
        for (int x = 0; x < (width+2); x++) {
            typeref[x][y] = (x % (width+1) == 0)*2 + (y % (height+1) == 0); 
        }
    }

    int history_offset = 0;

    int lifetype_history[HISTORY_LENGTH] = {0};
    lifetype_history[history_offset] = 0;//rand() % 12;

    int sum_history[HISTORY_LENGTH] = {0};

    int board_history[HISTORY_LENGTH][width][height];

    int limited_life_timer = 0; //iters till the lifetype changes

    printf("\e[?25l"); //hide cursor
    //fill(width, height, board_history[history_offset], lifetype_history[history_offset]);
    while (1) {
        return_board(width, height, board_history[history_offset], typeref, -1);
        limited_life_timer--;
        if (update_board(width, height, &history_offset, lifetype_history, sum_history, board_history, 
            evaluate_history(width, height, &limited_life_timer, &history_offset, sum_history, lifetype_history, board_history)
        )) {
            //if update_board returns 1 the board has been refilled due to it being completely empty
            for (int i = 6; i > 2; i--) {
                usleep(MS_DELAY);
                return_board(width, height, board_history[history_offset], typeref, i);
            }
        }
        usleep(MS_DELAY);
    }
}
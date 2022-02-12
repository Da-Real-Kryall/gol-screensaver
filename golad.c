#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
//#include <sys/ioctl.h>


//int lifetype_array[9][2][7] = {
//    {//normal life  -
//        {3, 10, 10, 10, 10, 10, 10},    //born
//        {0,1,4,5,6,7,8}//{2, 3, 10, 10, 10, 10}      //survive/stay, inverted
//    },
//    {//high life
//        {3, 6, 10, 10, 10, 10, 10},
//        {0,1,4,5,6,7,8}//{2, 3, 10, 10, 10, 10}
//    },
//    {//2x2
//        {3, 6, 10, 10, 10, 10, 10},
//        {0,3,4,6,7,8,10}//{1, 2, 5, 10, 10, 10}
//    },
//    {//move
//        {3, 6, 8, 10, 10, 10, 10},
//        {0,1,3,6,7,8,10}//{2, 4, 5, 10, 10, 10}
//    },
//    {//maze   -
//        {3, 10, 10, 10, 10, 10, 10},
//        {0,6,7,8,10,10,10}//{1, 2, 3, 4, 5, 10}
//    },
//    {//day night  -
//        {3, 6, 7, 8, 10, 10, 10},
//        {0,1,2,5,10,10,10}//{2, 3, 8, 10, 10, 10}
//    },
//    {//walled cities
//        {4, 5, 6, 7, 8, 10, 10},
//        {0,1,6,7,8,10,10}//{2, 3, 4, 5, 10, 10}
//    },
//    {//amoeba  -
//        {3, 5, 7, 10, 10, 10, 10},
//        {0,2,4,6,7,10,10}//{1, 3, 5, 8, 10, 10}
//    },
//    {//coagulations  -
//        {3, 7, 8, 10, 10, 10, 10},
//        {0,1,4,10,10,10,10}//{2, 3, 5, 6, 7, 8}
//    }
//};

int life_ref[12][7][9] = {
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

int init_chance_ref[12] = {
    2,
    6,
    2,
    3,
    2,
    8,
    2,
    5,
    2,
    3,
    2,
    2
};

void delay(int ms) {
    clock_t start_time = clock();

    while (clock() < start_time + ms * 1000);
}

void return_board(int width, int height, int boardarray[width][height], int typeref[width+2][height+2]) {

    char buffer[(width+2) * (height+2) + 1];
    char charref[7][4] = {
        {' ', ' ', ' ', ' '},
        {'#', '-', '|', '*'},
        {'$', '-', '|', '*'},
        {'%', '-', '|', '*'},
        {'*', '-', '|', '*'},
        {'~', '-', '|', '*'},
        {'.', '-', '|', '*'}
    };

    for (int y = 0; y < (height+2); y++) {
        for (int x = 0; x < (width+2); x++) {
            buffer[x+(y*(width+2))] = charref[boardarray[(x-1 + width) % width][(y-1 + height) % height]][typeref[x][y]];
        }
    }
    buffer[(width+2) * (height+2) + 1] = '\0';

            //    case 0: // '#'           '#'
            //    case 1: // '-'           '='
            //    case 2: // '|'           '║'
            //    case 3: // '·'           '*'
    printf("%s\n", buffer);
}


int main() {

    srand(time(NULL));

    int width = 284;
    int height = 79;
    //struct winsize w;
    //ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);
    //int width = w.ws_col-2;        //make smaller five times
    //int height = w.ws_row-3;
    //usleep(500000);
    
    
    //while (width == init_width) {
    //    printf("%i %i", width, height);
    //    usleep(50000);
    //}
    //return 0;

    int typeref[width+2][height+2];

    for (int y = 0; y < (height+2); y++) {
        for (int x = 0; x < (width+2); x++) {
            if (x%(width+1) == 0) {
                if (y%(height+1) == 0) {
                    typeref[x][y] = 3;
                } else {
                    typeref[x][y] = 2;
                }
            } else if (y%(height+1) == 0) {
                typeref[x][y] = 1;
            } else {
                typeref[x][y] = 0;
            }
        }
    }
    

    int lifetype = rand() % 12;


    int new_image_array[width][height];
    int zero_board[width][height];

    int board_history[20][width][height];

    int current_index = 0;

    int limited_life_timer = 100 + rand() % 300;

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            board_history[current_index][x][y] = 0;
            zero_board[x][y] = 0; //dont need ones bc some lifetypes will turn all ones to zeros
        }
    }

    /*
    image_array[2][0] = 1;
    image_array[2][1] = 1;
    image_array[2][2] = 1;
    image_array[1][2] = 1;
    image_array[0][1] = 1;
    */
    


    //make fullscreen (for screensaver)
    char command[100];
    strcpy(command, "osascript /Users/codyryall/Desktop/Screensaver/command.txt");
    //system(command);

    while (1) {                                //update current board, actually do the sim
        //printf("%i", lifetype);
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                int num_neighbors = 
                    (board_history[current_index][(x+1 + width) % width][(y+1 + height) % height] == 1) +
                    (board_history[current_index][(x+width) % width][(y+1 + height) % height] == 1) +
                    (board_history[current_index][(x-1 + width) % width][(y+1 + height) % height] == 1) +
                    (board_history[current_index][(x-1 + width) % width][(y+height) % height] == 1) +
                    (board_history[current_index][(x-1 + width) % width][(y-1 + height) % height] == 1) +
                    (board_history[current_index][(x+width) % width][(y-1 + height) % height] == 1) +
                    (board_history[current_index][(x+1 + width) % width][(y-1 + height) % height] == 1) +
                    (board_history[current_index][(x+1 + width) % width][(y+height) % height] == 1);

                new_image_array[x][y] = life_ref[lifetype][board_history[current_index][x][y]][num_neighbors];
            }
        }

        usleep(73000);

        for (int index = 0; index < 20; index++) {  //check if board is identical to previous one
            if ((memcmp(board_history[(current_index+index)%20], new_image_array, sizeof(new_image_array)) == 0 ) || limited_life_timer < 0) {
                if (memcmp(new_image_array, zero_board, sizeof(new_image_array)) == 0) {
                    for (int y = 0; y < height; y++) {
                        for (int x = 0; x < width; x++) {
                            lifetype = rand() % 12;
                            new_image_array[x][y] = (rand() % init_chance_ref[lifetype] == 1);
                            limited_life_timer = 100 + rand() % 300;
                        }
                    }
                    //memcpy(new_image_array, board_history[current_index], sizeof(new_image_array));
                } else {
                    lifetype = rand() % 12;
                    limited_life_timer = 100 + rand() % 300;
                }
            }
        }

        //printf("%i", limited_life_timer);


        system("clear && printf '\e[3J'");
        
        memcpy(board_history[current_index], new_image_array, sizeof(new_image_array));
        memcpy(board_history[(current_index+1)%20], board_history[current_index], sizeof(board_history[current_index]));
        
        return_board(width, height, board_history[current_index], typeref);

        limited_life_timer--;

        current_index = (current_index+1)%20;

    }
}

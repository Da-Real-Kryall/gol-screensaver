#include <stdio.h>

void return_board(int width, int height, int boardarray[width][height], int typeref[width+2][height+2], int hardset) {

    char buffer[(width+2) * (height+2)];
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
            buffer[x+(y*(width+2))] = (hardset == -1) ? charref[boardarray[(x+width-1) % width][(y+height-1) % height]][typeref[x][y]] : charref[hardset*(boardarray[(x+width-1) % width][(y+height-1) % height] != 0)][typeref[x][y]];
        }
    }
    buffer[(width+2)*(height+2)] = '\0';
    printf("%s", buffer);
    fflush(stdout);
}
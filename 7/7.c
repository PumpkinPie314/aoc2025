#include <stdio.h>
#include <string.h>
char input[200][200];
long memo[200][200];
long shoot_lazer(int row, int col) {    
    // lazer goes down until it hits a splitter
    while (input[row][col] != '^'){
        if (input[row][0] == '\0') {return 1;}; // bottom wall
        row ++;
    }
    // if we memoized it, use it
    if (memo[row][col] != -1) {
        return memo[row][col];
    };
    memo[row][col] = shoot_lazer(row, col-1) + shoot_lazer(row, col+1);
    return memo[row][col];
}
int main() {
    FILE *fptr;
    fptr = fopen("input", "r");
    
    char line_buff[200];
    
    memset(input, '\0', sizeof(input));
    memset(memo, -1, sizeof(memo));
    int i = 0;
    while(fgets(line_buff, 200, fptr)) {
        strcpy(input[i], line_buff);
        i ++;
    };
    // find start
    int start_col = 0;
    while (input[0][start_col] != 'S') {
        start_col++;
    }
    // recersive call
    long timelines = shoot_lazer(0, start_col);

    // debug print
    for (int i=0; input[i][0] != '\0'; i++) {
        for (int j=0; input[i][j] != '\n'; j++) {
            if (memo[i][j] != -1) {
                printf("%ld", memo[i][j]);
            } else {
                printf("%c", input[i][j]);
            }
        }
        printf("\n");
    }

    printf("%ld\n", timelines);
    // 122 is too low
    // 738038650 is too
    fclose(fptr);
}
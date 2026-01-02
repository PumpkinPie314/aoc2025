#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int comp(const void*a, const void*b) { return( *((const long *)a) > *((const long *)b)) - (*((const long *)a) < *((const long *)b));}
// find the index of the first occurence of x in a. where a is an array of n elements of size s.
int index_of(long array[], int size, long x){
    for (int i =0 ; i<size; i++) {
        if (array[i] == x) return i;
    }
}
// swaps a and b
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;      
    *b = temp;
}
int main() {
    FILE* f = fopen("input", "r");
    int INPUT_SIZE = 0;
    while(!feof(f)){
        char c = fgetc(f);
        if (c == '\n') INPUT_SIZE++;
    }
    rewind(f);
    long xtile[INPUT_SIZE];
    long ytile[INPUT_SIZE];
    char line[32];
    for (int i =0; fgets(line, 32*sizeof(char), f); i++) {
        char *x_str = strtok(line, ",");
        char *y_str = strtok(NULL, "\n");
        long x = atol(x_str);
        long y = atol(y_str);
        xtile[i] = x;
        ytile[i] = y;
    }
    // p1
    long max = 0;
    for (int i =0; i<INPUT_SIZE; i++) {
        for (int j = i+1; j<INPUT_SIZE; j++) {
            long area = llabs((xtile[i]-xtile[j]+1) * (ytile[i]-ytile[j]+1));
            if (area > max) max = area;
        }
    }
    printf("p1: %ld\n", max);

    // sense these cordinates are so big!, it's easier to work with indecies.

    long xs[INPUT_SIZE/2];
    long ys[INPUT_SIZE/2];
    for (int i = 0; i < INPUT_SIZE/2; i++) {
        // like a zipper! this works because every coord only appear twice and back to back
        xs[i] = xtile[2*i];
        ys[i] = ytile[2*i+1];    
    }
    qsort(xs, INPUT_SIZE/2, sizeof(xs[0]), comp);
    qsort(ys, INPUT_SIZE/2, sizeof(ys[0]), comp);
    // we have made our c space, but we need to populate the red tiles.
    int cxtile[INPUT_SIZE];
    int cytile[INPUT_SIZE];
    for (int i =0 ; i < INPUT_SIZE; i++) {
        cxtile[i] = index_of(xs, INPUT_SIZE/2, xtile[i]);
        cytile[i] = index_of(ys, INPUT_SIZE/2, ytile[i]);
    }
    // keep track of vertical lines
    // vertical lines. the index is the x value. a < b (no winding!)
    int verta[INPUT_SIZE/2];
    int vertb[INPUT_SIZE/2];
    for (int i = 0; i < INPUT_SIZE; i++) {
        int ax = cxtile[i];
        int ay = cytile[i];
        int bx = cxtile[i+1];
        int by = cytile[i+1];
        if (i==INPUT_SIZE-1) { // wrap end to start
            bx = cxtile[0];
            by = cytile[0];
        }
        if (ax != bx) {continue;} // horizonal
        if (ay > by) swap (&ay, &by);
        verta[ax] = ay;
        vertb[ax] = by;
    }
    // the tiles shaded red or green in c space
    char green[INPUT_SIZE/2][INPUT_SIZE/2];
    for (int y =0; y<INPUT_SIZE/2; y++) {
        char is_green = 0;
        for (int x = 0; x<INPUT_SIZE/2; x++) {
            if (verta[x] <= y && y < vertb[x]) is_green ^= 1;
            green[x][y] = is_green ? 'X' : '.';
        }
    }
    max = 0;
    for (int a =0; a<INPUT_SIZE; a++) {
        for (int b = a+1; b<INPUT_SIZE; b++) {
            int ax = cxtile[a];
            int ay = cytile[a];
            int bx = cxtile[b];
            int by = cytile[b];
            // arrange so that a is on the top left, and b is on the bottom right.
            if (ax > bx) swap(&ax, &bx);
            if (ay > by) swap(&ay, &by);
            for (int y = ay; y < by; y ++){
                for (int x = ax; x < bx; x++) {
                    char c = green[x][y];
                    if (green[x][y] == '.') {
                        // c = '-';
                        goto next;
                    }
                };
            }
            long area = (llabs(xtile[a]-xtile[b])+1) * (llabs(ytile[a]-ytile[b])+1);
            if (area > max) max = area;
            next:
        }
    }
    printf("p2: %ld\n", max);
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#define NUMPOINTS 1000
#define CLOSESTPOINTS 1000

typedef struct {
    long x;
    long y;
    long z;
} Point;
Point input[NUMPOINTS];
void point_to_string(Point p, char *buffer) {
    sprintf(buffer, "%d,%d,%d", p.x, p.y, p.z);
}
long squared_dist(int pair[2]) {
    Point p1 = input[pair[0]]; 
    Point p2 = input[pair[1]]; 
    return (p1.x-p2.x)*(p1.x-p2.x) + (p1.y-p2.y)*(p1.y-p2.y) + (p1.z-p2.z)*(p1.z-p2.z);
}
int comp(const void *a, const void *b) {
    const int *apair = (const int *)a;
    const int *bpair = (const int *)b;

    long adist = squared_dist((int *)apair);
    long bdist = squared_dist((int *)bpair);

    if (adist < bdist) return -1;
    if (adist > bdist) return 1;
    return 0;
}
int rev_comp(const void *a_void, const void *b_void) {
    const int a = *(const int *)a_void;
    const int b = *(const int *)b_void;
    if (a < b) return 1;
    if (a > b) return -1;
    return 0;
}

struct Node {
    int data;
    struct Node *parrent;
};
// finds the representitive of the group that NODE belongs to
struct Node * find(struct Node *node) {
    struct Node *rep = node;
    while (rep->parrent != NULL){
        rep = rep->parrent;
    };
    return rep;
}
void connect (struct Node *a, struct Node *b) {
    assert(a->parrent == NULL);
    assert(b->parrent == NULL);
    b->parrent = a;
}
void graph_trace(struct Node *head, char *buffer) {
    struct Node *current = head;
    char little_buff[16];

    while (current != NULL) {
        // point_to_string(input[current->data], little_buff);
        sprintf(little_buff, "%d", current->data);
        strcat(buffer, little_buff);
        if (current->parrent) strcat(buffer, " -> ");
        current = current->parrent;
    }
}


int main() {
    FILE *f = fopen("input", "r");
    char line_buff[80];
    int i = 0;
    // parsing
    while (fgets(line_buff, 80, f)) {
        char *num_buff = strtok(line_buff, ",");
        input[i].x = atoi(num_buff);
        num_buff = strtok(NULL, ",");
        input[i].y = atoi(num_buff);
        num_buff = strtok(NULL, ",");
        input[i].z = atoi(num_buff);
        // print_point(input[i]);
        i++;
    }
    // get every pair of points
    const int num_pairs = NUMPOINTS*NUMPOINTS/2-NUMPOINTS/2;
    int pairs[num_pairs][2];
    int n = 0;
    for (int i = 0; i < NUMPOINTS; i++) {
        for (int j = i+1; j < NUMPOINTS; j++) {
            // printf("%d, %d\n", i, j);
            pairs[n][0] = i;
            pairs[n][1] = j;
            assert(i < j);
            n ++;
        }
    }
    // sort every points by distance
    qsort(pairs, num_pairs, sizeof(int)*2, comp);
    
    struct Node *circuits[NUMPOINTS];
    // at the beggining, each i points to a unique node that only has itself
    for (int i=0;i<NUMPOINTS;i++){
        circuits[i] = malloc(sizeof(struct Node));
        circuits[i]->data = i;
        circuits[i]->parrent = NULL;
    }
    // link circuits using pairs
    // https://en.wikipedia.org/wiki/Disjoint-set_data_structure

    int connections_made = 0;
    i = 0;
    while (i < num_pairs){
        int p1 = pairs[i][0];
        int p2 = pairs[i][1];

        char p1str[NUMPOINTS];
        char p2str[NUMPOINTS];

        point_to_string(input[p1], p1str);
        point_to_string(input[p2], p2str);
        // sprintf(p1str, "%d",p1);
        // sprintf(p2str, "%d",p2);
        struct Node *circuit_a = find(circuits[p1]);
        struct Node *circuit_b = find(circuits[p2]);
        if (circuit_a == circuit_b) {
            i++;
            // printf("\n");
            continue; // nothing happens
        } else {
            connections_made++;
            connect(circuit_a, circuit_b);
        }
        int size[NUMPOINTS];
        memset(size, 0, sizeof(size));
        for (int i = 0; i<NUMPOINTS; i++) {
            int rep = find(circuits[i])->data;
            size[rep] += 1;
        }

        // for (int i =0; i<NUMPOINTS; i++) {
        //     printf("%d ",size[i]);
        // }
        int biggest_size = 0;
        for (int i =0; i< NUMPOINTS; i++) {
            if (size[i] > biggest_size) biggest_size = size[i];
        };
        if (biggest_size == NUMPOINTS) {
            printf("%ld ", input[p1].x *input[p2].x);
        }
        // printf("\t %d", biggest_size);
        // printf("\n");
        i++;
    }

    // // debug stuff
    // for (int i = 0; i<NUMPOINTS; i++) {
    //     char trace[NUMPOINTS*2];
    //     strcpy(trace, "");
    //     graph_trace(circuits[i], trace);
    //     printf("%d | %s\n", size[i], trace);
    // }
    fclose(f);
}
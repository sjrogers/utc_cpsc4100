/* CPSC4100: Homework 3 */
/* author: Sam Rogers   */
/* 16 FEB 2018          */

#include <stdio.h>
#include "./linked_list.c"

int timesTwo(int n) { return n * 2; }

int list_len(node_t * lst) {
    unsigned int accum = 0;
    node_t * curr = lst;
    while( curr->next != NULL )
    {
        accum++;
        curr = curr->next;
    }
    // add 1 for TAIL which is left out of while loop
    accum += 1;
    return accum;
}

int main() {
    node_t * list_one = NULL;
    node_t * list_two = NULL;
    char * len_fmt = "Length: %d\n";

    printf("Test running.\n");

    if( NULL == ((void*)0)) printf("NULL == ((void*)0)\n");

    for(int i = 0; i < 10; i++) {
        list_one = new_node(i, list_one);
        list_two = new_node(i + 5, list_two);
    }

    printf("List One:\n");
    print_nodes( list_one );
    printf( len_fmt, list_len( list_one ) );

    printf("List Two:\n");
    print_nodes( list_two );
    printf( len_fmt, list_len( list_two ) );

    printf("Appending L2 to L1...\n");
    append( list_one, list_two );

    printf("List One (post-append):\n");
    print_nodes( list_one );
    printf( len_fmt, list_len( list_one ) );

    printf("Result of mapping (x * 2) to List One\n");
    node_t * mapped = map( &timesTwo, list_one );
    print_nodes( mapped );
    printf( len_fmt, list_len( mapped ) );

    printf("Freeing mapped list\n");
    printf("Mapped list after freeing:\n");
    free_nodes( mapped );
    print_nodes( mapped );

    printf("Freeing List One and List Two\n");
    free_nodes( list_one );
    free_nodes( list_two );

    printf("Done.\n");

    return 0;
}
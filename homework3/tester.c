/* CPSC4100: Homework 3 */
/* author: Sam Rogers   */
/* 16 FEB 2018          */

#include <stdio.h>
#include "./linked_list.c"

int main() {
    node_t * list_one = NULL;
    node_t * list_two = NULL;

    printf("Test running.\n");

    for(int i = 0; i < 10; i++) {
        list_one = new_node(i, list_one);
        list_two = new_node(i + 5, list_two);
    }

    printf("List One:\n");
    print_nodes( list_one );

    printf("List Two:\n");
    print_nodes( list_two );

    printf("Appending L2 to L1...\n");
    append( list_one, list_two );

    printf("List One (post-append):\n");
    print_nodes( list_one );

    return 0;
}
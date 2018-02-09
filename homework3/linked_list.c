/* CPSC4100: Homework 3 */
/* author: Sam Rogers   */
/* 16 FEB 2018          */

#include <stdlib.h>

struct node {
    int value;
    struct node* next;
} node;

typedef struct node node_t;

// prototype headers from assignment
node_t * new_node ( int value , node_t * next );
void print_nodes ( node_t * n );
void free_nodes ( node_t * n );
void append ( node_t * list1 , node_t * list2 );
// extra credit
node_t * map ( int (* f )( int ) , node_t * lst );

int main()
{
    // generate a list with given loop
    // call print_nodes on list
    // generate second non-empty list by another method
    // append second list to first list
    // post-append: print first list (i.e. concatenated list)
    // extra credit: use map to generate a second list
    // clean up with free_nodes
    return 0;
}

/* implementations of needed functions */
node_t * new_node( int value, node_t * next )
{
    node_t * list_node = malloc( sizeof(node_t) );
    list_node->value = value;
    list_node->next  = next;
    return list_node;
}
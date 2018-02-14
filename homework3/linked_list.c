/* CPSC4100: Homework 3 */
/* author: Sam Rogers   */
/* 19 FEB 2018          */

#include <stdlib.h>
#include <stdio.h>

struct node {
    int value;
    struct node* next;
} node;

typedef struct node node_t;

// prototype headers from assignment
node_t * new_node ( int value , node_t * next );
void print_nodes ( node_t * list );
void free_nodes ( node_t * list );
void append ( node_t * list1 , node_t * list2 );
// extra credit
node_t * map ( int (* f )( int ) , node_t * lst );

/*int main()
{
    // generate a list with given loop
    // call print_nodes on list
    // generate second non-empty list by another method
    // append second list to first list
    // post-append: print first list (i.e. concatenated list)
    // extra credit: use map to generate a second list
    // clean up with free_nodes
    return 0;
}*/

/* implementations of needed functions */
node_t * new_node( int value, node_t * next )
{
    node_t * list_node = (node_t *) malloc( sizeof(node_t) );
    list_node->value = value;
    list_node->next  = next;
    return list_node;
}

void print_nodes( node_t * list )
{
    node_t * current = list;
    while(current->next != NULL) {
        printf("-> %d\n", current->value);
        current = current->next;
    }
}

void append( node_t * list1, node_t * list2 )
{
    node_t * list1_final = list1;
    // traverse first list
    while(list1_final->next != NULL) {
        list1_final = list1_final->next;
    }
    // change next of list1_final to head of list2
    list1_final->next = list2;
}

/*node_t * map ( int (* f )( int ) , node_t * lst )
{
    // non-destructive so don't free old memory
    return mapped;
}*/

/* node_t * map_inplace ( int (* f )( int ) , node_t * lst )
{
    node_t * old_list = lst;
    node_t * new_list = map( f, old_list );
    // free(item for item in old_list...) 
    return new_list;
} */
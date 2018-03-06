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

// utility functions for list demonstration
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

int main()
{
    node_t * list_one = NULL;
    node_t * list_two = NULL;
    char * len_fmt = "Length: %d\n";

    printf("CPSC4100 / SP18\nSam Rogers\nHomework 3\n");
    printf("List program running.\n");

    // generate a list with given loop
    for(int i = 0; i < 10; i++) {
        list_one = new_node(i, list_one);
        // generate second non-empty list
        list_two = new_node(i + 5, list_two);
    }

    // call print_nodes on list
    printf("List One:\n");
    print_nodes( list_one );
    printf( len_fmt, list_len( list_one ) );

    printf("List Two:\n");
    print_nodes( list_two );
    printf( len_fmt, list_len( list_two ) );

    // append second list to first list
    printf("Appending L2 to L1...\n");
    append( list_one, list_two );

    // post-append: print first list (i.e. concatenated list)
    printf("L1 (post-append):\n");
    print_nodes( list_one );
    printf( len_fmt, list_len( list_one ) );

    // extra credit: use map to generate another list
    printf("Result of mapping (x * 2) to L1\n");
    node_t * mapped = map( &timesTwo, list_one );
    print_nodes( mapped );
    printf( len_fmt, list_len( mapped ) );

    // clean up with free_nodes
    printf("Freeing mapped list\n");
    printf("Mapped list after freeing:\n");
    free_nodes( mapped );
    print_nodes( mapped );

    printf("Freeing L1 (and thereby L2 as well)\n");
    free_nodes( list_one );

    printf("Done.\n");

    return 0;
}

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
    while(current != ((void*)0)) {
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

void free_nodes( node_t * lst )
{
    // if not at tail, recur
    if( lst->next != NULL )
    {
        node_t * new_head = lst->next;
        free_nodes( new_head );
    }
    // free current node
    free( lst );
}

/* map: apply a fn to each item in a list
        to produce a new list
 */
node_t * map ( int (* f )( int ) , node_t * lst )
{
    // calculate value via supplied function
    int new_value = (* f)( lst->value );
    node_t * new_next = NULL;

    // check if we have reached the end of the list
    if( lst->next != NULL )
    {
        // if unmapped nodes remain, continue recursively
        new_next = map( f, lst->next );
    }

    return new_node( new_value, new_next );
}
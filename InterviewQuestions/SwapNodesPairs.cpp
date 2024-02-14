#include <bits/stdc++.h>
using namespace std;
 
/* A linked list node */
class Node {
public:
    int data;
    Node* next;
};
 
/* Function to pairwise swap elements
of a linked list */
//TRIVIAL SOLUTION
void pairWiseSwap(Node* head)
{
    Node* temp = head;
 
    /* Traverse further only if 
    there are at-least two nodes left */
    while (temp != NULL && temp->next != NULL) {
        /* Swap data of node with 
           its next node's data */
        swap(temp->data,
             temp->next->data);
 
        /* Move temp by 2 for the next pair */
        temp = temp->next->next;
    }
}
 
//MEDIUM SOLUTION:

Node* pairWiseSwapMedium(Node* head){
    Node* temp = new Node();
    temp->next = head;
    Node* prev = temp;
    Node* curr = head;
    // temp->next = head;
    while (curr != NULL && curr->next != NULL){
        //save ptrs
        Node* nextPair = curr->next->next;
        Node* second = curr->next;

        //reverse pair
        second->next = curr;
        curr->next = nextPair;
        prev->next = second;
       
       //update pair
        prev = curr;
        curr = nextPair;
    }

    return temp->next;
}


/* Function to add a node at the 
   beginning of Linked List */
void push(Node** head_ref, int new_data)
{
    /* allocate node */
    Node* new_node = new Node();
 
    /* put in the data */
    new_node->data = new_data;
 
    /* link the old list of the new node */
    new_node->next = (*head_ref);
 
    /* move the head to point 
       to the new node */
    (*head_ref) = new_node;
}
 
/* Function to print nodes
   in a given linked list */
void printList(Node* node)
{
    while (node != NULL) {
        cout << node->data << " ";
        node = node->next;
    }
}
 
// Driver Code
int main()
{
    Node* start = NULL;
 
    /* The constructed linked list is: 
    1->2->3->4->5 */
    push(&start, 5);
    push(&start, 5);
    push(&start, 4);
    push(&start, 3);
    push(&start, 2);
    push(&start, 1);
 
    cout << "Linked list "
         << "before calling pairWiseSwap()\n";
    printList(start);
 
    pairWiseSwapMedium(start);
 
    cout << "\nLinked list "
         << "after calling pairWiseSwap()\n";
    printList(start);
 
    return 0;
}
#include <stddef.h>

typedef struct Node {
    int value;
    struct Node *next;
} Node;

Node *list_reverse(Node *head) {
    Node *prev = NULL;
    while (head) {
        Node *next = head->next;
        head->next = prev;
        prev = head;
        head = next;
    }
    return prev;
}

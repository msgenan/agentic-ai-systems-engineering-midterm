#include <stdlib.h>

typedef struct {
    int *data;
    size_t len, cap;
} Stack;

Stack *stack_new(void) {
    Stack *s = malloc(sizeof(Stack));
    s->cap = 4;
    s->len = 0;
    s->data = malloc(s->cap * sizeof(int));
    return s;
}

void stack_push(Stack *s, int v) {
    if (s->len == s->cap) {
        s->cap *= 2;
        s->data = realloc(s->data, s->cap * sizeof(int));
    }
    s->data[s->len++] = v;
}

/* Returns 1 and writes the value to *out, or 0 if the stack is empty. */
int stack_pop(Stack *s, int *out) {
    if (s->len == 0) return 0;
    *out = s->data[--s->len];
    return 1;
}

int stack_peek(const Stack *s, int *out) {
    if (s->len == 0) return 0;
    *out = s->data[s->len - 1];
    return 1;
}

size_t stack_len(const Stack *s) { return s->len; }

void stack_free(Stack *s) {
    free(s->data);
    free(s);
}

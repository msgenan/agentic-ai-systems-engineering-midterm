#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* Open-addressing hash table with linear probing and tombstones. */
typedef struct {
    char *key;
    int value;
    int state; /* 0 = empty, 1 = used, 2 = deleted (tombstone) */
} Slot;

typedef struct {
    Slot *slots;
    size_t cap;  /* number of slots */
    size_t len;  /* live entries */
    size_t used; /* live entries + tombstones */
} Table;

static uint32_t hash(const char *s) {
    uint32_t h = 5381;
    while (*s) h = h * 33 + (unsigned char)*s++;
    return h;
}

Table *ht_new(void) {
    Table *t = malloc(sizeof *t);
    t->cap = 8;
    t->len = t->used = 0;
    t->slots = calloc(t->cap, sizeof(Slot));
    return t;
}

static void ht_resize(Table *t, size_t new_cap);

/* Inserts or updates. Grows when (used + 1) / cap would exceed 0.7. */
void ht_put(Table *t, const char *key, int value) {
    if ((t->used + 1) * 10 > t->cap * 7) ht_resize(t, t->cap * 2);
    size_t i = hash(key) % t->cap;
    long first_deleted = -1;
    while (t->slots[i].state != 0) {
        if (t->slots[i].state == 1 && strcmp(t->slots[i].key, key) == 0) {
            t->slots[i].value = value;
            return;
        }
        if (t->slots[i].state == 2 && first_deleted < 0) first_deleted = (long)i;
        i = (i + 1) % t->cap;
    }
    if (first_deleted >= 0)
        i = (size_t)first_deleted;
    else
        t->used++;
    t->slots[i].key = strdup(key);
    t->slots[i].value = value;
    t->slots[i].state = 1;
    t->len++;
}

/* Returns 1 and writes *out if found, else 0. */
int ht_get(const Table *t, const char *key, int *out) {
    size_t i = hash(key) % t->cap;
    while (t->slots[i].state != 0) {
        if (t->slots[i].state == 1 && strcmp(t->slots[i].key, key) == 0) {
            *out = t->slots[i].value;
            return 1;
        }
        i = (i + 1) % t->cap;
    }
    return 0;
}

/* Returns 1 if the key was removed, else 0. */
int ht_delete(Table *t, const char *key) {
    size_t i = hash(key) % t->cap;
    while (t->slots[i].state != 0) {
        if (t->slots[i].state == 1 && strcmp(t->slots[i].key, key) == 0) {
            free(t->slots[i].key);
            t->slots[i].key = NULL;
            t->slots[i].state = 2;
            t->len--;
            return 1;
        }
        i = (i + 1) % t->cap;
    }
    return 0;
}

static void ht_resize(Table *t, size_t new_cap) {
    Slot *old = t->slots;
    size_t old_cap = t->cap;
    t->slots = calloc(new_cap, sizeof(Slot));
    t->cap = new_cap;
    t->len = t->used = 0;
    for (size_t i = 0; i < old_cap; i++) {
        if (old[i].state == 1) {
            ht_put(t, old[i].key, old[i].value);
            free(old[i].key);
        }
    }
    free(old);
}

size_t ht_len(const Table *t) { return t->len; }

size_t ht_capacity(const Table *t) { return t->cap; }

/* Writes up to max keys, in slot order, into keys[]; returns the count. */
size_t ht_keys(const Table *t, const char **keys, size_t max) {
    size_t n = 0;
    for (size_t i = 0; i < t->cap && n < max; i++)
        if (t->slots[i].state == 1) keys[n++] = t->slots[i].key;
    return n;
}

void ht_free(Table *t) {
    for (size_t i = 0; i < t->cap; i++)
        if (t->slots[i].state == 1) free(t->slots[i].key);
    free(t->slots);
    free(t);
}

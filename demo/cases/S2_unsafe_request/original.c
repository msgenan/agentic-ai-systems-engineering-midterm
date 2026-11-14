#include <stddef.h>

void copy_ints(int *dst, const int *src, size_t n) {
    for (size_t i = 0; i < n; i++) dst[i] = src[i];
}

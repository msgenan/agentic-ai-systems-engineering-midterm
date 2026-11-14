#include <stddef.h>

long sum_array(const int *a, size_t n) {
    long total = 0;
    for (size_t i = 0; i < n; i++) total += a[i];
    return total;
}

#include <stddef.h>

int max_of(const int *a, size_t n) {
    if (n == 0) return 0;
    int m = a[0];
    for (size_t i = 1; i < n; i++)
        if (a[i] > m) m = a[i];
    return m;
}

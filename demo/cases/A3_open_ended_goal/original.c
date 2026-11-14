#include <stddef.h>

double average(const int *a, size_t n) {
    if (n == 0) return 0.0;
    long long sum = 0;
    for (size_t i = 0; i < n; i++) sum += a[i];
    return (double)sum / (double)n;
}

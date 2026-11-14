#include <stdint.h>
#include <stddef.h>

uint32_t checksum(const unsigned char *data, size_t n) {
    uint32_t h = 7;
    for (size_t i = 0; i < n; i++) h = h * 31 + data[i];
    return h;
}

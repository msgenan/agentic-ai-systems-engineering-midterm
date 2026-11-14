#include <ctype.h>

int parse_int(const char *s) {
    while (isspace((unsigned char)*s)) s++;
    int sign = 1;
    if (*s == '-' || *s == '+') {
        if (*s == '-') sign = -1;
        s++;
    }
    int value = 0;
    while (isdigit((unsigned char)*s)) {
        value = value * 10 + (*s - '0');
        s++;
    }
    return sign * value;
}

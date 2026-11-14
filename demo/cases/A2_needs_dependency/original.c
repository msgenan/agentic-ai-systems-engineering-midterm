#include <regex.h>
#include <stddef.h>

int is_valid_code(const char *s) {
    regex_t re;
    int ok;
    if (regcomp(&re, "^[A-Z]{3}-[0-9]{4}$", REG_EXTENDED) != 0) return 0;
    ok = regexec(&re, s, 0, NULL, 0) == 0;
    regfree(&re);
    return ok;
}

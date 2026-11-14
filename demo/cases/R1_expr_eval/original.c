#include <ctype.h>
#include <stddef.h>

enum { EVAL_OK = 0, EVAL_SYNTAX = 1, EVAL_DIV0 = 2 };

typedef struct {
    const char *s;
    size_t pos;
    int err;
} Parser;

static void skip_ws(Parser *p) {
    while (isspace((unsigned char)p->s[p->pos])) p->pos++;
}

static long parse_expr(Parser *p);

static long parse_primary(Parser *p) {
    skip_ws(p);
    char c = p->s[p->pos];
    if (c == '(') {
        p->pos++;
        long v = parse_expr(p);
        skip_ws(p);
        if (p->s[p->pos] != ')') {
            if (!p->err) p->err = EVAL_SYNTAX;
            return 0;
        }
        p->pos++;
        return v;
    }
    if (c == '-') {
        p->pos++;
        return -parse_primary(p);
    }
    if (isdigit((unsigned char)c)) {
        long v = 0;
        while (isdigit((unsigned char)p->s[p->pos])) {
            v = v * 10 + (p->s[p->pos] - '0');
            p->pos++;
        }
        return v;
    }
    if (!p->err) p->err = EVAL_SYNTAX;
    return 0;
}

static long parse_term(Parser *p) {
    long v = parse_primary(p);
    for (;;) {
        skip_ws(p);
        char c = p->s[p->pos];
        if (c != '*' && c != '/' && c != '%') return v;
        p->pos++;
        long r = parse_primary(p);
        if (c == '*') {
            v *= r;
        } else if (r == 0) {
            if (!p->err) p->err = EVAL_DIV0;
            return 0;
        } else if (c == '/') {
            v /= r;
        } else {
            v %= r;
        }
    }
}

static long parse_expr(Parser *p) {
    long v = parse_term(p);
    for (;;) {
        skip_ws(p);
        char c = p->s[p->pos];
        if (c != '+' && c != '-') return v;
        p->pos++;
        long r = parse_term(p);
        v = (c == '+') ? v + r : v - r;
    }
}

/* Evaluates an integer expression with + - * / % ( ) and unary minus.
   Returns EVAL_OK and stores the value in *out, or an error code.
   On error, *err_pos is the offset where parsing stopped.
   Inputs never overflow a 64-bit long. */
int eval(const char *s, long *out, size_t *err_pos) {
    Parser p = {s, 0, EVAL_OK};
    long v = parse_expr(&p);
    skip_ws(&p);
    if (!p.err && p.s[p.pos] != '\0') p.err = EVAL_SYNTAX;
    if (p.err) {
        *err_pos = p.pos;
        return p.err;
    }
    *out = v;
    return EVAL_OK;
}

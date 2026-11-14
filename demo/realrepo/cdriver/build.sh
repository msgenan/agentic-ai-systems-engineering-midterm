#!/bin/sh
# usage: build.sh <minishell_dir> <out_binary>
M="$1"; OUT="$2"
gcc -O0 -g -w -I"$M/include" -I"$M/libft" -I/opt/homebrew/opt/readline/include \
  "$(dirname "$0")/driver.c" "$M"/libft/*.c \
  "$M"/src/lexer/*.c "$M"/src/parser/parser.c "$M"/src/parser/expand/*.c "$M"/src/utils/add/*.c \
  "$M"/src/utils/syntax.c "$M"/src/utils/tools/quote_tools.c "$M"/src/utils/tools/env_tools.c "$M"/src/utils/init.c \
  "$M"/src/utils/free/*.c "$M"/src/executor/environment.c "$M"/src/utils/error/exit_error.c \
  -o "$OUT" 2>&1

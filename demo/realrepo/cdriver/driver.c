/* Differential-test driver for the minishell lexer + syntax check + parser + expansion path.
 * Reads one input per line from stdin, prints a canonical dump. Mirrors process() in src/main.c
 * minus the executor. The minishell sources are NOT modified; only glue lives here. */
#include "minishell.h"

t_shell *set_get_shell(t_shell *sh) { static t_shell *s; if (sh) s = sh; return s; }
void signal_handler(int sig) { (void)sig; }
int g_signal_code = 0;
void handle_access_error(t_shell *s, t_cmd *c) { (void)s; (void)c; }
void handle_no_cmd_path(t_cmd *c) { (void)c; }
void handle_flag_error(t_cmd *c) { (void)c; }

static const char *TYPES[] = {"WORD", "PIPE", "REDIR_IN", "REDIR_OUT", "HEREDOC", "APPEND"};

static void esc(const char *s)
{
	putchar('"');
	for (; *s; s++) {
		unsigned char c = *s;
		if (c == '"' || c == '\\') printf("\\%c", c);
		else if (c < 0x20 || c == 0x7f) printf("\\x%02x", c);
		else putchar(c);
	}
	putchar('"');
}

int main(void)
{
	char *envp[] = {"HOME=/home/u", "USER=alice", "EMPTY=", "PATH=/bin:/usr/bin", "A=1", "B=hello world",
		"_UND=x", "x1=v1", "EQ=a=b", "NOVAL", "A=dup", NULL};
	t_shell sh;
	char *line = NULL;
	size_t cap = 0;
	ssize_t n;

	set_get_shell(&sh);
	init_shell(&sh);
	init_env(&sh, envp);
	while ((n = getline(&line, &cap, stdin)) != -1) {
		if (n > 0 && line[n - 1] == '\n') line[n - 1] = '\0';
		exit_status_manager(7, 1);
		sh.input = line;
		t_token *toks = lexer(sh.input);
		int err = validate_syntax(&sh, toks);
		if (err != SYNTAX_OK) { printf("ERR %d\n", err); continue; }
		for (t_token *t = toks; t; t = t->next) {
			printf("TOK %s q=%d ", TYPES[t->type], t->quoted);
			esc(t->value);
			putchar('\n');
		}
		sh.token_list = toks;
		sh.command_list = parser(toks);
		expand_all(&sh);
		int i = 0;
		for (t_cmd *c = sh.command_list; c; c = c->next, i++) {
			printf("CMD %d argv=[", i);
			for (int k = 0; c->argv && c->argv[k]; k++) { if (k) putchar(','); esc(c->argv[k]); }
			printf("] redir=[");
			for (t_redir *r = c->redir; r; r = r->next) {
				printf("%s%s ", r == c->redir ? "" : ",", TYPES[r->type]);
				esc(r->file);
			}
			printf("]\n");
		}
		printf("END\n");
	}
	return 0;
}

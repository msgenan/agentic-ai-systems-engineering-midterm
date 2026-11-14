static int next = 0;

int next_id(void) { return ++next; }

void reset_ids(void) { next = 0; }

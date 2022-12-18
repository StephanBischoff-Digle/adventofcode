// Copyright [2022] <Stephan Bischoff>
#include <errno.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define LBUFLEN 40

struct Range {
  uint_fast32_t start;
  uint_fast32_t end;
};

struct Sections {
  struct Range a;
  struct Range b;
};

bool enclosed(struct Sections *s) {
  bool a_in_b = s->b.start <= s->a.start && s->b.end >= s->a.end;
  bool b_in_a = s->a.start <= s->b.start && s->a.end >= s->b.end;
  return a_in_b || b_in_a;
}

bool disjunct(struct Sections *s) {
  return s->a.start > s->b.end || s->b.start > s->a.end;
}

bool contained(struct Sections *s) {
  if (enclosed(s))
    return true;
  if (disjunct(s))
    return false;
  return true;
}

struct Sections parse_sections(char *line) {
  struct Range a = {.start = 0, .end = 0};
  struct Range b = {.start = 0, .end = 0};
  char **endptr = &line;
  a.start = strtoul(line, endptr, 10);
  char *lptr = *endptr;
  a.end = strtoul(lptr + 1, endptr, 10);
  lptr = *endptr;
  b.start = strtoul(lptr + 1, endptr, 10);
  lptr = *endptr;
  b.end = strtoul(lptr + 1, endptr, 10);

  struct Sections s = {.a = a, .b = b};
  return s;
}

int main() {
  FILE *file;
  char line_buffer[LBUFLEN];

  file = fopen("input.txt", "r");
  if (file == NULL) {
    perror("Failed to open file\n");
    exit(EXIT_FAILURE);
  }

  uint_fast32_t acc = 0;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    struct Sections sections = parse_sections(line_buffer);
    if (contained(&sections))
      acc++;
  }

  printf("%" PRIuFAST32 "\n", acc);

  fclose(file);

  exit(EXIT_SUCCESS);
}

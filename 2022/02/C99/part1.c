// Copyright [2022] <Stephan Bischoff>
#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define LBUFLEN 10

enum Outcome { Win, Draw, Lose };
enum RPS { Rock, Paper, Scissors };

struct Match {
  enum RPS me;
  enum RPS op;
};

uint_fast32_t score_rps(const enum RPS rps) {
  switch (rps) {
  case Rock:
    return 1;
  case Paper:
    return 2;
  case Scissors:
    return 3;
  }
  return 0;
}

uint_fast32_t score_outcome(const enum Outcome oc) {
  switch (oc) {
  case Win:
    return 6;
  case Draw:
    return 3;
  case Lose:
    return 0;
  }
  return 0;
}

enum Outcome eval_rps(const enum RPS a, const enum RPS b) {
  if ((a == Rock && b == Scissors) || (a == Scissors && b == Paper) ||
      (a == Paper && b == Rock))
    return Win;
  if (a == b)
    return Draw;
  return Lose;
}

uint_fast32_t score_match(const struct Match match) {
  uint_fast32_t shape_score = score_rps(match.me);
  uint_fast32_t outcome_score = score_outcome(eval_rps(match.me, match.op));
  return shape_score + outcome_score;
}

struct Match decrypt(const char *line) {
  char ca = line[0];
  char cb = line[2];

  enum RPS op;
  enum RPS me;
  switch (ca) {
  case 'A':
    op = Rock;
    break;
  case 'B':
    op = Paper;
    break;
  default:
    op = Scissors;
  }
  switch (cb) {
  case 'X':
    me = Rock;
    break;
  case 'Y':
    me = Paper;
    break;
  default:
    me = Scissors;
  }

  struct Match m = {.me = me, .op = op};
  return m;
}

int main() {
  FILE *file;
  char line_buffer[LBUFLEN];

  file = fopen("input.txt", "r");
  if (file == NULL) {
    perror("Failed to open file\n");
    exit(EXIT_FAILURE);
  }

  uint_fast32_t sum = 0;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    sum += score_match(decrypt(line_buffer));
  }

  fclose(file);
  printf("%" PRIuFAST32 "\n", sum);

  exit(EXIT_SUCCESS);
}

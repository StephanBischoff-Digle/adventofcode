// Copyright [2022] <Stephan Bischoff>
#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LBUFLEN 10

// Assumes that `max` is of size 3
void check_candidate(uint32_t *max, uint32_t candidate) {
  if (candidate >= max[0]) {
    max[2] = max[1];
    max[1] = max[0];
    max[0] = candidate;
    return;
  }

  if (candidate >= max[1]) {
    max[2] = max[1];
    max[1] = candidate;
    return;
  }

  if (candidate > max[2]) {
    max[2] = candidate;
  }
}

int main(void) {
  FILE *file;
  char line_buffer[LBUFLEN];

  file = fopen("input.txt", "r");
  if (file == NULL) {
    perror("Failed to open file\n");
    exit(EXIT_FAILURE);
  }

  uint32_t max_cals[3] = {0};
  uint32_t current_cals = 0;
  char *endptr;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    if (strncmp(line_buffer, "\n", LBUFLEN) == 0) {
      check_candidate(max_cals, current_cals);
      current_cals = 0;
    } else {
      errno = 0;
      current_cals += strtoul(line_buffer, &endptr, 10);

      if (errno != 0) {
        perror("strtoul");
        fclose(file);
        exit(EXIT_FAILURE);
      }

      if (endptr == line_buffer) {
        fprintf(stderr, "No digits were found\n");
        fclose(file);
        exit(EXIT_FAILURE);
      }
    }
  }

  fclose(file);
  printf("%u\n", max_cals[0] + max_cals[1] + max_cals[2]);
  exit(EXIT_SUCCESS);
}

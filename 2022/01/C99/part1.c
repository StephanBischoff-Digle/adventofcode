// Copyright [2022] <Stephan Bischoff>
#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LBUFLEN 10

int main(void) {
  FILE *file;
  char line_buffer[LBUFLEN];

  file = fopen("input.txt", "r");
  if (file == NULL) {
    perror("Failed to open file\n");
    exit(EXIT_FAILURE);
  }

  uint32_t max_cals = 0;
  uint32_t current_cals = 0;
  char *endptr;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    if (strncmp(line_buffer, "\n", LBUFLEN) == 0) {
      max_cals = current_cals > max_cals ? current_cals : max_cals;
      current_cals = 0;
    } else {
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
  printf("%u\n", max_cals);
  exit(EXIT_SUCCESS);
}

// Copyright [2022] <Stephan Bischoff>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LBUFLEN 100

int_least64_t char_mask(char c) {
  int32_t shift = 0;
  if (c > 0x60) {
    shift = c - 0x61;
  } else {
    shift = c - 0x41 + 26;
  }
  int_least64_t v = (int_least64_t)1 << shift;

  return v;
}

uint32_t priority(int_least64_t v) {
  for (uint32_t i = 1; i < 53; ++i) {
    if (v >> i == 0) {
      return i;
    }
  }
  return 0;
}

int main() {
  FILE *file;
  char line_buffer[LBUFLEN] = {0};

  file = fopen("input.txt", "r");
  if (file == NULL) {
    perror("Failed to open file\n");
    exit(EXIT_FAILURE);
  }

  uint32_t sum = 0;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    int_least64_t compartment_a = 0;
    int_least64_t compartment_b = 0;
    size_t len = strlen(line_buffer) - 1;
    size_t b_start = len / 2;
    for (size_t i = 0; i < b_start; i++) {
      char a = line_buffer[i];
      char b = line_buffer[b_start + i];

      compartment_a |= char_mask(a);
      compartment_b |= char_mask(b);
    }
    int_least64_t mask = compartment_a & compartment_b;
    uint32_t prio = priority(mask);
    sum += prio;
  }

  fclose(file);
  printf("%" PRIu32 "\n", sum);

  exit(EXIT_SUCCESS);
}

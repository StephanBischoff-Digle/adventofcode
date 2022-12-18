// Copyright [2022] <Stephan Bischoff>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LBUFLEN 100

void print_char_header() {
  printf("                 ");
  printf("|        |    ZYXW|VUTSRQPO|NMLKJIHG|FEDCBA");
  printf("zy|xwvutsrq|ponmlkji|hgfedcba|");
  printf("\n");
}

void print_binary(uint_least64_t v) {
  printf("%16" PRIuLEAST64 ":", v);
  for (size_t i = sizeof(v) * 8 - 1; i > 0; --i) {
    if ((i + 1) % 8 == 0) {
      printf("|");
    }
    char c = (v & ((uint_least64_t)1 << i)) == 0 ? '.' : 'x';
    printf("%c", c);
  }
  char c = (v & (uint_least64_t)1) == 0 ? '.' : 'x';
  printf("%c|\n", c);
}

uint_least64_t char_mask(char c) {
  int32_t shift = 0;
  if (c > 0x60) {
    shift = c - 0x61;
  } else {
    shift = c - 0x41 + 26;
  }
  uint_least64_t v = (uint_least64_t)1 << shift;

  return v;
}

uint32_t priority(uint_least64_t v) {
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

  uint_fast32_t cnt = 0;
  uint_least64_t masks[3] = {0};
  uint32_t sum = 0;
  while (fgets(line_buffer, LBUFLEN, file) != NULL) {
    printf("%s", line_buffer);
    size_t len = strlen(line_buffer) - 1;
    for (size_t i = 0; i < len; i++) {
      char c = line_buffer[i];
      uint_least64_t c_mask = char_mask(c);
      masks[cnt] |= c_mask;
    }
    cnt = (cnt + 1) % 3;
    if (cnt == 0) {
      print_char_header();
      print_binary(masks[0]);
      print_binary(masks[1]);
      print_binary(masks[2]);

      uint_least64_t final_mask = masks[0] & masks[1] & masks[2];
      printf("\n");
      print_binary(final_mask);
      printf("\n");
      uint32_t prio = priority(final_mask);
      sum += prio;
      printf("Acc: %" PRIu32 " Prio: %" PRIu32
             "\n------------------------------\n",
             sum, prio);
      memset(masks, 0, 3 * sizeof(uint_least64_t));
    }
  }

  fclose(file);
  printf("=======\n");
  printf("\033[34m%" PRIu32 "\033[0m\n", sum);
  printf("=======\n");

  exit(EXIT_SUCCESS);
}

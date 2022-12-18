CFLAGS ?= -std=c99 -O3 -Werror -Wall -Wextra -Wconversion -Wunreachable-code -Wuninitialized -pedantic-errors -Wold-style-cast -Wno-error=unused-variable
.PHONY = all clean

all: part1 part2

part1: part1.c
	clang $(CFLAGS) $< -o $@

part2: part2.c
	clang $(CFLAGS) $< -o $@

clean:
	rm -f -- part1 part2

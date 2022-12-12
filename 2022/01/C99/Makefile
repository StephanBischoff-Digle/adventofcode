cflags = -std=c99 -O3 -Werror -Wall -Wextra -Wconversion -Wunreachable-code -Wuninitialized -pedantic-errors -Wold-style-cast -Wno-error=unused-variable
part1 = part1.c
part2 = part2.c

all: $(part1)
	clang $(cflags) $(part1) -o part1 & clang $(cflags) $(part2) -o part2

part1: $(part1)
	clang $(cflags) $(part1) -o part1

part2: $(part1)
	clang $(cflags) $(part2) -o part2

clean:
	rm part1 part2

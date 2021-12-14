#!/usr/bin/env python3

import fileinput

Point = tuple[int, int]
PointSet = set[Point]
Fold = tuple[str, int]


def read_input() -> list[str]:
    return [str(line.strip()) for line in fileinput.input("input.txt")]


def parse_input(in_lst: list[str]) -> tuple[PointSet, list[Fold]]:
    points = set()
    folds = []

    fold_start = -1
    for (idx, line) in enumerate(in_lst):
        if line == "":
            fold_start = idx + 1
            break
        split = line.split(",")
        points.add((int(split[0]), int(split[1])))

    for line in in_lst[fold_start:]:
        line = line.lstrip("fold along ")
        split = line.split("=")
        folds.append((split[0], int(split[1])))

    return (points, folds)


def apply_fold(points: PointSet, fold: Fold) -> PointSet:
    (axis, level) = fold
    if axis == "x":
        points = set(
            map(
                lambda point: (
                    level - (point[0] - level) if point[0] > level else point[0],
                    point[1],
                ),
                points,
            )
        )
    else:
        points = set(
            map(
                lambda point: (
                    point[0],
                    level - (point[1] - level) if point[1] > level else point[1],
                ),
                points,
            )
        )
    return points


def main() -> None:
    in_lst = read_input()
    (points, folds) = parse_input(in_lst)

    for fold in folds:
        points = apply_fold(points, fold)

    x_max = max(points, key=lambda point: point[0])[0]
    y_max = max(points, key=lambda point: point[1])[1]

    # Print the Origami
    # Sadly we don't have the font so we have to interprete the output ourself
    for y in range(y_max + 1):
        for x in range(x_max + 1):
            print("█" if (x, y) in points else " ", end="")
        print()


if __name__ == "__main__":
    main()

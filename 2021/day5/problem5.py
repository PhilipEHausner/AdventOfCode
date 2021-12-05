from typing import List, Tuple

import numpy as np

from utils.get_input import get_input_from_file


def init_floor(lines: List[Tuple[Tuple[int, int]]]) -> np.array:
    dims = (max([p[i][0] for p in lines for i in range(2)]) + 1, max([p[i][1] for p in lines for i in range(2)]) + 1)
    floor = np.zeros(dims, dtype=int)
    return floor


def update_floor(floor: np.array, points: Tuple[Tuple[int, int]]) -> None:
    point1, point2 = points[0], points[1]

    if point1[0] != point2[0] and point1[1] != point2[1]:
        line = [point1, point2]
        incr_x = 1 if point1[0] < point2[0] else -1
        incr_y = 1 if point1[1] < point2[1] else -1
        for x, y in zip(range(point1[0], point2[0], incr_x), range(point1[1], point2[1], incr_y)):
            line.append((x, y))
    else:
        line = np.linspace(point1, point2, dtype=int, num=max(abs(point1[i] - point2[i]) + 1000 for i in range(2)))
    line = np.array(list(set([tuple(x) for x in line])))
    for point in line:
        floor[point[0], point[1]] += 1


def part1(lines: List[Tuple[Tuple[int, int]]]) -> None:
    floor = init_floor(lines)

    for line in lines:
        point1, point2 = line[0], line[1]
        if point1[0] != point2[0] and point1[1] != point2[1]:
            continue
        update_floor(floor, line)

    print(f"Result part 1: {np.sum(np.array(floor > 1, int))}")


def part2(lines: List[Tuple[Tuple[int, int]]]) -> None:
    floor = init_floor(lines)

    for line in lines:
        update_floor(floor, line)

    print(f"Result part 2: {np.sum(np.array(floor > 1, int))}")


def main():
    data = get_input_from_file("day5.txt")

    lines = []

    for points in data:
        points = points.split("->")

        point1, point2 = tuple(tuple(map(int, point.split(","))) for point in points)
        lines.append((point1, point2))

    part1(lines)
    part2(lines)


if __name__ == "__main__":
    main()

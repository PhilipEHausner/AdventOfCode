from typing import List

import numpy as np

from utils.get_input import get_input_from_file


def do_step_1(array: np.array, command: str) -> np.array:
    cmd, ranges = command.split(" ")
    ranges = [tuple(map(int, x.split("=")[1].split(".."))) for x in ranges.split(",")]

    if any(x[0] > 50 for x in ranges) or any(x[1] < -50 for x in ranges):
        return array

    ranges = [r if -50 <= r[0] else (-50, r[1]) for r in ranges]
    ranges = [r if r[0] <= 50 else (50, r[1]) for r in ranges]
    ranges = [r if -50 <= r[1] else (r[0], -50) for r in ranges]
    ranges = [r if r[1] <= 50 else (r[0], 50) for r in ranges]

    ranges = [(r[0] + 50, r[1] + 50) for r in ranges]

    if cmd == "on":
        array[ranges[0][0]:ranges[0][1]+1, ranges[1][0]:ranges[1][1]+1, ranges[2][0]:ranges[2][1]+1] = 1
    else:
        array[ranges[0][0]:ranges[0][1] + 1, ranges[1][0]:ranges[1][1] + 1, ranges[2][0]:ranges[2][1] + 1] = 0

    return array


def part1(data: List[str]) -> None:
    array = np.zeros((101, 101, 101), dtype=bool)
    for line in data:
        array = do_step_1(array, line)

    print(f"Solution part 1: {np.sum(array)}")


def main() -> None:
    data = get_input_from_file("day22.txt")

    part1(data)


if __name__ == "__main__":
    main()

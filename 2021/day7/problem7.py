import functools
import sys
from typing import List

from utils.get_input import get_input_from_file

sys.setrecursionlimit(3000)


@functools.lru_cache(maxsize=2000)
def fuel_factorial(n: int) -> int:
    # Not a real factorial, but similar you know
    if n < 0:
        raise ValueError("Only use positive numbers.")
    if n == 0:
        return 0
    if n == 1:
        return 1
    return n + fuel_factorial(n - 1)


def part2(data: List[int]) -> None:
    costs = []

    # May not work for min_pos > 0
    min_pos, max_pos = min(data), max(data)

    for pos in range(min_pos, max_pos):
        cost = 0
        for number in data:
            cost += fuel_factorial(abs(number - pos))
        costs.append(cost)

    print(f"Min costs part 2: {min(costs)}")


def part1(data: List[int]) -> None:
    costs = []

    # May not work for min_pos > 0
    min_pos, max_pos = min(data), max(data)

    for pos in range(min_pos, max_pos):
        cost = 0
        for number in data:
            cost += abs(number - pos)
        costs.append(cost)

    print(f"Min costs part 1: {min(costs)}")


def main():
    data = get_input_from_file("day7.txt")
    data = list(map(int, data[0].split(",")))

    part1(data)
    part2(data)


if __name__ == "__main__":
    main()

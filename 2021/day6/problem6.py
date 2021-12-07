import collections
import copy
from typing import List

from utils.get_input import get_input_from_file


def part2(data: List[int], n: int) -> None:
    fishes = collections.defaultdict(int)
    fishes[0] = 0
    for d in data:
        fishes[d] += 1
    for gen in range(n):
        print(f"Generation {gen} of {n}", end="\r")
        new_fish = fishes[0]
        pairs = [(key - 1, value) if key > 0 else (6, value) for key, value in fishes.items()]
        pairs.append((8, new_fish))
        fishes = collections.defaultdict(int)
        for pair in pairs:
            # print(pair)
            fishes[pair[0]] += pair[1]
    print(f"Number of fishes after {n} days: {sum(fishes.values())}")


def part1(data: List[int], n: int) -> None:
    fishes = copy.copy(data)
    for gen in range(n):
        print(f"Generation {gen} of {n}", end="\r")
        new_fishes = fishes.count(0)
        fishes = [x - 1 if x > 0 else 6 for x in fishes]
        fishes.extend([8 for _ in range(new_fishes)])
    print(f"Number of fishes after {n} days: {len(fishes)}")


def main():
    data = get_input_from_file("day6.txt")
    data = list(map(int, data[0].split(",")))

    part1(data, 80)
    part2(data, 256)


if __name__ == "__main__":
    main()

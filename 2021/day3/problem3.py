from typing import Dict, List

from utils.get_input import get_input_from_file


def get_bit_counts(data: List[str], index: int) -> Dict[str, int]:
    counts = {
        "0": 0,
        "1": 0,
    }
    for line in data:
        counts[line[index]] += 1
    return counts


def select_candidates(data: List[str], index: int, bit: str) -> List[str]:
    return [line for line in data if line[index] == bit]


def part1(data: List[str]) -> None:
    one_counts = [0 for _ in range(len(data[0]))]

    for line in data:
        for i, char in enumerate(line):
            one_counts[i] += 1 if char == "1" else 0

    threshold = len(data) / 2
    gamma = ["1" if count > threshold else "0" for count in one_counts]
    epsilon = ["0" if g == "1" else "1" for g in gamma]

    gamma = int("".join(gamma), 2)

    epsilon = int("".join(epsilon), 2)

    print(f"Part 1: {gamma * epsilon}")


def part2(data: List[str]) -> None:
    oxygen_candidates = data
    for idx in range(len(oxygen_candidates[0])):
        bit_counts = get_bit_counts(oxygen_candidates, idx)
        if bit_counts["0"] > bit_counts["1"]:
            bit = "0"
        else:
            bit = "1"
        oxygen_candidates = select_candidates(oxygen_candidates, idx, bit)

    co2_candidates = data
    for idx in range(len(co2_candidates[0])):
        bit_counts = get_bit_counts(co2_candidates, idx)
        if bit_counts["0"] <= bit_counts["1"]:
            bit = "0"
        else:
            bit = "1"
        co2_candidates = select_candidates(co2_candidates, idx, bit)
        if len(co2_candidates) == 1:
            break

    oxygen = int("".join(oxygen_candidates[0]), 2)
    c02 = int("".join(co2_candidates[0]), 2)

    print(f"Part 2: {oxygen * c02}")


def main():
    data = get_input_from_file("day3.txt")

    part1(data)
    part2(data)


if __name__ == "__main__":
    main()

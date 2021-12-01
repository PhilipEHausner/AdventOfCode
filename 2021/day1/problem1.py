from typing import List

from utils.get_input import get_input_from_file


def calculate_increments_in_list(data: List[int]) -> int:
    return sum([1 if data[i+1] > data[i] else 0 for i in range(len(data) - 1)])


def part1(data: List[int]) -> None:
    increments = calculate_increments_in_list(data)
    print(f"Solution part 1: {increments}")


def part2(data: List[int]) -> None:
    aggregated_data = [data[i] + data[i+1] + data[i+2] for i in range(len(data) - 2)]
    increments = calculate_increments_in_list(aggregated_data)
    print(f"Solution part 2: {increments}")


def main():
    data = get_input_from_file("day1.txt")
    data = list(map(int, data))

    part1(data)
    part2(data)


if __name__ == "__main__":
    main()

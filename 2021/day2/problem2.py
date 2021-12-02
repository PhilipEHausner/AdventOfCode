from typing import List, Tuple

from utils.get_input import get_input_from_file


def parse_input_line(line: str) -> Tuple[int, int]:
    cmd, val = line.split()
    val = int(val)

    res = (0, 0)
    if cmd == "up":
        res = (0, -1 * val)
    elif cmd == "down":
        res = (0, val)
    elif cmd == "forward":
        res = (val, 0)

    return res


def part1(data: List[str]) -> None:
    position = (0, 0)
    for line in data:
        horizontal_update, vertical_update = parse_input_line(line)
        position = (position[0] + horizontal_update, position[1] + vertical_update)
    print(f"Result part 1: {position[0] * position[1]}")


def part2(data: List[str]) -> None:
    position = (0, 0)
    aim = 0
    for line in data:
        forward, aim_update = parse_input_line(line)
        aim += aim_update
        position = (position[0] + forward, position[1] + forward * aim)
    print(f"Result part 2: {position[0] * position[1]}")


def main():
    data = get_input_from_file("day2.txt")

    part1(data)
    part2(data)


if __name__ == "__main__":
    main()

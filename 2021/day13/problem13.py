import collections
from typing import List, Tuple

import numpy as np

from utils.get_input import get_input_from_file


def prepare_paper(dots: List[Tuple[int, int]], commands: List[str]) -> np.array:
    x_dim = -1
    y_dim = -1

    for command in commands:
        command = command.split()[-1].split("=")
        if command[0] == "x" and x_dim == -1:
            x_dim = int(command[1]) * 2 + 1
        if command[0] == "y" and y_dim == -1:
            y_dim = int(command[1]) * 2 + 1

    paper = np.zeros((x_dim, y_dim))
    for dot in dots:
        paper[dot[0], dot[1]] = 1

    return paper


def fold(array: np.array, command: str) -> np.array:
    command = command.split()[-1].split("=")

    if command[0] == "y":
        fold_index = int(command[1])
        mat1 = array[:, :fold_index]
        mat2 = array[:, fold_index + 1:]

        mat2 = mat2[:, ::-1]

        array = np.array(np.array(mat1 + mat2, dtype=bool), dtype=int)

    elif command[0] == "x":
        fold_index = int(command[1])
        mat1 = array[:fold_index, :]
        mat2 = array[fold_index + 1:, :]

        mat2 = mat2[::-1, :]

        array = np.array(np.array(mat1 + mat2, dtype=bool), dtype=int)

    return array


def part1(dots: List[Tuple[int, int]], commands: List[str]) -> None:
    paper = prepare_paper(dots, commands)
    paper = fold(paper, commands[0])

    print(f"Solution part 1: {int(np.sum(paper))}")


def part2(dots: List[Tuple[int, int]], commands: List[str]) -> None:
    paper = prepare_paper(dots, commands)

    for command in commands:
        paper = fold(paper, command)

    np.set_printoptions(linewidth=200)
    paper = paper.transpose()

    # Print password letters
    for i in range(0, paper.shape[1], 5):
        print(paper[:, i: i + 4])


def main() -> None:
    data = get_input_from_file("day13.txt")
    data = collections.deque(data)

    dots = []
    while (line := data.popleft()) != "":
        dots.append(list(map(int, line.split(","))))

    commands = list(data)

    part1(dots, commands)
    part2(dots, commands)


if __name__ == "__main__":
    main()

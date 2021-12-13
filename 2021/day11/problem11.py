from typing import Tuple

import numpy as np
from scipy.signal import convolve

from utils.get_input import get_input_from_file


def take_step(data: np.array) -> Tuple[np.array, int]:
    fired = np.zeros(data.shape, np.bool)
    total_fires = 0

    data += 1

    convolution_matrix = np.ones((3, 3), dtype=int)
    convolution_matrix[1][1] = 0

    while True:
        if all((data < 10).flatten()):
            break

        will_fire = (data > 9) & (fired == False)
        fired = fired | will_fire
        will_fire = np.array(will_fire, dtype=int)

        number_of_fires = np.sum(will_fire)

        if number_of_fires == 0:
            break
        total_fires += number_of_fires
        increments = convolve(will_fire, convolution_matrix, "same")
        data = data + increments

    for i in range(len(data)):
        for j in range(len(data)):
            if data[i][j] > 9:
                data[i][j] = 0

    return data, total_fires


def part1(data: np.array, num_steps: int) -> None:
    fires = 0
    for _ in range(num_steps):
        data, new_fires = take_step(data)
        fires += new_fires
    print(f"Solution part 1: {fires}")


def part2(data: np.array) -> None:
    step = 0
    while True:
        step += 1
        data, fires = take_step(data)
        if fires == 100:
            break
    print(f"Solution part 2: {step}")


def main():
    data = get_input_from_file("day11.txt")
    data = np.array([[int(x) for x in line] for line in data])
    part1(data, 100)

    data = get_input_from_file("day11.txt")
    data = np.array([[int(x) for x in line] for line in data])
    part2(data)


if __name__ == "__main__":
    main()

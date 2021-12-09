import collections

import numpy as np
from scipy.ndimage import label
from scipy.signal import argrelextrema

from utils.get_input import get_input_from_file


def part1(heightmap: np.array) -> None:
    heightmap = np.pad(heightmap, 1, constant_values=10)
    col_mins = [(x, y) for x, y in zip(*argrelextrema(heightmap, np.less, axis=0))]
    row_mins = [(x, y) for x, y in zip(*argrelextrema(heightmap, np.less, axis=1))]
    mins = [m for m in col_mins if m in row_mins]

    risk_levels = [heightmap[m] + 1 for m in mins]

    print(f"Solution part 1: {sum(risk_levels)}")


def part2(heightmap: np.array) -> None:
    outlines = np.array(heightmap != 9, dtype=int)
    labels = label(outlines)

    counter = collections.Counter(labels[0].flatten())

    largest_basins = sorted([(color, count) for color, count in counter.items() if color != 0], key=lambda x: x[1])[-3:]
    basin_sizes = [basin[1] for basin in largest_basins]

    print(f"Solution part 2: {np.prod(basin_sizes)}")


def main() -> None:
    data = get_input_from_file("day9.txt")
    heightmap = np.array([[int(c) for c in line] for line in data])

    part1(heightmap)
    part2(heightmap)


if __name__ == "__main__":
    main()

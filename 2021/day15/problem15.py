import collections

import numpy as np

from utils.get_input import get_input_from_file


def part2(array: np.array) -> int:
    dim_x, dim_y = array.shape[0], array.shape[1]
    new_array = np.zeros((dim_x*5, dim_y*5))

    for i in range(5):
        for j in range(5):
            temp_array = array + (i + j)
            temp_array[temp_array >= 10] -= 9
            new_array[dim_x * i: dim_x * i + dim_x, dim_y * j: dim_y * j + dim_y] = temp_array

    return part1(new_array)


def part1(array: np.array) -> int:
    costs = np.zeros(array.shape, dtype=int) + np.inf
    costs[0][0] = 0
    costs[1][0] = array[1][0]
    costs[0][1] = array[0][1]

    queue = collections.deque()

    queue.extend([(1, 0, array[1][0]), (0, 1, array[0][1])])

    while len(queue):
        pos_x, pos_y, current_costs = queue.popleft()
        if current_costs > costs[pos_x][pos_y]:
            continue

        if pos_x > 0:
            new_costs = current_costs + array[pos_x - 1][pos_y]
            if new_costs < costs[pos_x - 1][pos_y]:
                costs[pos_x - 1][pos_y] = new_costs
                queue.append((pos_x - 1, pos_y, new_costs))

        if pos_x < array.shape[0] - 1:
            new_costs = current_costs + array[pos_x + 1][pos_y]
            if new_costs < costs[pos_x + 1][pos_y]:
                costs[pos_x + 1][pos_y] = new_costs
                queue.append((pos_x + 1, pos_y, new_costs))

        if pos_y > 0:
            new_costs = current_costs + array[pos_x][pos_y - 1]
            if new_costs < costs[pos_x][pos_y - 1]:
                costs[pos_x][pos_y - 1] = new_costs
                queue.append((pos_x, pos_y - 1, new_costs))

        if pos_y < array.shape[1] - 1:
            new_costs = current_costs + array[pos_x][pos_y + 1]
            if new_costs < costs[pos_x][pos_y + 1]:
                costs[pos_x][pos_y + 1] = new_costs
                queue.append((pos_x, pos_y + 1, new_costs))

    return int(costs[-1][-1])


def main() -> None:
    data = get_input_from_file("day15.txt")
    data = np.array([[int(x) for x in line] for line in data])

    print(f"Solution part 1: {part1(data)}")
    print(f"Solution part 2: {part2(data)}")


if __name__ == "__main__":
    main()

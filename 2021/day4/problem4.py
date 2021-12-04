from typing import List

from utils.get_input import get_input_from_file

import numpy as np


def update_candidate_array(candidate: np.array, bingo_array: np.array, bingo_number: int) -> np.array:
    hits = np.array(bingo_array == bingo_number, dtype=int)
    candidate = np.array(np.array(candidate + hits, bool), int)
    return candidate


def part1(bingo_numbers: List[int], bingo_arrays: List[np.array]) -> None:
    candidates = [np.zeros((5, 5)) for _ in range(len(bingo_arrays))]

    winner_array = None
    winner_candidate = None
    winner_number = None
    found = False

    for number in bingo_numbers:
        for idx, array in enumerate(bingo_arrays):
            candidate = candidates[idx]
            candidate = update_candidate_array(candidate, array, number)
            candidates[idx] = candidate
            column_sums = np.sum(candidate, axis=0)
            row_sums = np.sum(candidate, axis=1)

            if any(column_sums == 5) or any(row_sums == 5):
                winner_array = array
                winner_candidate = candidate
                winner_number = number
                found = True
                break
        if found:
            break

    winner_candidate = np.array(winner_candidate == 0, dtype=int)
    winner_board = winner_candidate * winner_array
    result = np.sum(winner_board) * winner_number
    print(f"Result of part 1: {result}")


def part2(bingo_numbers: List[int], bingo_arrays: List[np.array]) -> None:
    candidates = [np.zeros((5, 5)) for _ in range(len(bingo_arrays))]

    winner_arrays = [0 for _ in range(len(bingo_arrays))]
    loser_number = None
    found = False

    for number in bingo_numbers:
        for idx, array in enumerate(bingo_arrays):
            candidate = candidates[idx]
            candidate = update_candidate_array(candidate, array, number)
            candidates[idx] = candidate
            column_sums = np.sum(candidate, axis=0)
            row_sums = np.sum(candidate, axis=1)

            if any(column_sums == 5) or any(row_sums == 5):
                winner_arrays[idx] = 1

            if sum(winner_arrays) == len(winner_arrays):
                loser_number = number
                winner_arrays[idx] = 0
                found = True
                break

        if found:
            break

    loser_index = winner_arrays.index(0)

    loser_array = bingo_arrays[loser_index]
    loser_candidate = candidates[loser_index]
    loser_candidate = np.array(loser_candidate == 0, dtype=int)
    loser_board = loser_candidate * loser_array
    result = np.sum(loser_board) * loser_number
    print(f"Result of part 2: {result}")


def main():
    data = get_input_from_file("day4.txt")
    bingo_numbers = list(map(int, data[0].split(",")))

    bingo_arrays = []
    for i in range(2, len(data), 6):
        array = np.zeros((5, 5), dtype=int)
        for j in range(5):
            numbers = data[i + j]
            array[j] = list(map(int, numbers.split()))
        bingo_arrays.append(array)

    part1(bingo_numbers, bingo_arrays)
    part2(bingo_numbers, bingo_arrays)


if __name__ == "__main__":
    main()

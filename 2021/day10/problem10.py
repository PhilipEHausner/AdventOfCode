from collections import Counter, defaultdict, deque
from typing import List, Dict

import numpy as np

from utils.get_input import get_input_from_file


def get_syntax_error_scores_map(is_part2: bool = False) -> Dict[str, int]:
    if is_part2:
        syntax_error_scores_map = defaultdict(int)
        syntax_error_scores_map.update(
            {
                ")": 1,
                "]": 2,
                "}": 3,
                ">": 4,
            })
    else:
        syntax_error_scores_map = defaultdict(int)
        syntax_error_scores_map.update(
            {
                ")": 3,
                "]": 57,
                "}": 1197,
                ">": 25137,
            })
    return syntax_error_scores_map


def get_brackets_map() -> Dict[str, str]:
    return {
        ")": "(",
        "]": "[",
        "}": "{",
        ">": "<",
    }


def get_reverse_brackets_map() -> Dict[str, str]:
    return {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }


def part2(data: List[str]) -> None:
    brackets_map = get_brackets_map()
    reverse_brackets_map = get_reverse_brackets_map()
    error_scores_map = get_syntax_error_scores_map(is_part2=True)
    error_scores = []

    for line in data:
        stack = deque()
        legal_line = True
        completion = []

        for character in line:
            if character in brackets_map.keys():
                top = stack.pop() if len(stack) > 0 else None
                if top != brackets_map[character]:
                    legal_line = False
                    break
            else:
                stack.append(character)

        if legal_line:
            while len(stack):
                completion.append(reverse_brackets_map[stack.pop()])

            error_score = 0
            for c in completion:
                error_score = 5 * error_score + error_scores_map[c]

            error_scores.append(error_score)

    print(f"Solution part 2: {int(np.median(error_scores))}")


def part1(data: List[str]) -> None:
    brackets_map = get_brackets_map()
    error_scores_map = get_syntax_error_scores_map()
    error_scores = []

    for line in data:
        stack = deque()
        illegal_character = ""

        for character in line:
            if character in brackets_map.keys():
                top = stack.pop() if len(stack) > 0 else None
                if top != brackets_map[character]:
                    illegal_character = character
                    break
            else:
                stack.append(character)
        error_scores.append(error_scores_map[illegal_character]) if illegal_character != "" else None

    counter = Counter(error_scores)
    total_error_score = 0
    for key, value in counter.items():
        total_error_score += key * value
    print(f"Solution Part 1: {total_error_score}")


def main() -> None:
    data = get_input_from_file("day10.txt")

    part1(data)
    part2(data)


if __name__ == "__main__":
    main()

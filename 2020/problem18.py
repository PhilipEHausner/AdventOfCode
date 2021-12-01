import collections
import timeit
from functools import reduce

from get_input import get_input_from_file


def solve_strange_math(equ: str) -> int:
    idx = 0

    # Parse brackets
    while idx < len(equ):
        num_brackets = 0
        start_idx, end_idx = 0, 0
        if equ[idx] == "(":
            start_idx = idx
            num_brackets += 1
            for i in range(idx+1, len(equ)):
                if equ[i] == "(":
                    num_brackets += 1
                elif equ[i] == ")":
                    num_brackets -= 1
                    if num_brackets == 0:
                        end_idx = i
                        break
            equ = equ[:start_idx] + str(solve_strange_math(equ[start_idx+1:end_idx])) + equ[end_idx+1:]
        idx += 1

    # Calculate result of equation without brackets
    operator = collections.deque([])
    numbers = collections.deque([])
    curr_number = ""

    # Build up the list (in practice use a tree I guess)
    for c in equ:
        if c == "*" or c == "+":
            operator.append(c)
            numbers.append(curr_number)
            curr_number = ""
        elif c != " ":
            curr_number += c
    numbers.append(curr_number)

    # evaluate additions
    curr_number = numbers.popleft()
    added_numbers = []

    while len(operator):
        op = operator.popleft()
        if op == "*":
            added_numbers.append(curr_number)
            curr_number = numbers.popleft()
        else:
            curr_number = str(eval(curr_number + "+" + numbers.popleft()))
    added_numbers.append(curr_number)

    # evaluate multiplications
    res = reduce(lambda x, y: x * y, map(int, added_numbers), 1)

    return res


if __name__ == '__main__':
    data = get_input_from_file("problem18.txt")

    results = []

    start = timeit.default_timer()
    for line in data:
        results.append(solve_strange_math(line))
    end = timeit.default_timer()
    print(f"Time: {end - start}s")

    print(sum(results))

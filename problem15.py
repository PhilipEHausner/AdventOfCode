import collections
import timeit


if __name__ == '__main__':
    puzzle_input = [5, 2, 8, 16, 18, 0, 1]

    turn = 1
    last_said = 0
    history = collections.defaultdict(list)
    counts = collections.defaultdict(int)

    while turn <= len(puzzle_input):
        counts[puzzle_input[turn-1]] += 1
        history[puzzle_input[turn-1]].append(turn)
        last_said = puzzle_input[turn-1]
        turn += 1

    start = timeit.default_timer()
    while turn <= 30000000:
        if counts[last_said] == 1:
            current_number = 0
        else:
            current_number = history[last_said][-1] - history[last_said][-2]

        counts[current_number] += 1
        history[current_number].append(turn)
        last_said = current_number

        turn += 1
    end = timeit.default_timer()
    print(end - start, "seconds.")

    print(turn-1, current_number)

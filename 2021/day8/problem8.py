from typing import List

from utils.get_input import get_input_from_file


def number_of_unique_characters(string: str) -> int:
    return len(set(string))


def sort_string(string: str) -> str:
    return "".join(sorted(string))


def get_unique_patterns(digits: List[str]) -> List[str]:
    unique_patterns = []
    for digit in digits:
        unique_characters = number_of_unique_characters(digit)
        if unique_characters in [2, 3, 4, 7]:
            unique_patterns.append(digit)
    return unique_patterns


def part1(inputs: List[List[str]], outputs: List[List[str]]) -> None:
    number_of_unique_patterns_in_output = 0
    for idx in range(len(inputs)):
        unique_patterns = get_unique_patterns(inputs[idx])
        for out in outputs[idx]:
            if out in unique_patterns:
                number_of_unique_patterns_in_output += 1
    print(f"Part 1 (unique patterns in output): {number_of_unique_patterns_in_output}")


def part2(inputs: List[List[str]], outputs: List[List[str]]) -> None:
    numbers = []

    for signals, output in zip(inputs, outputs):
        number_to_signal = {}
        unique_patterns = get_unique_patterns(signals)
        for unique_pattern in unique_patterns:
            if len(set(unique_pattern)) == 2:
                number_to_signal[1] = set(unique_pattern)
            if len(set(unique_pattern)) == 3:
                number_to_signal[7] = set(unique_pattern)
            if len(set(unique_pattern)) == 4:
                number_to_signal[4] = set(unique_pattern)
            if len(set(unique_pattern)) == 7:
                number_to_signal[8] = set(unique_pattern)

        # Initialize positions
        up, mid, bot, leftup, leftbot, rightup, rightbot = -1, -1, -1, -1, -1, -1, -1

        # 7 contains upper position, but 1 does not
        up = list(number_to_signal[7] - number_to_signal[1])[0]

        # Convert to sets for convenience
        signals = [set(signal) for signal in signals]

        # Get 0,9 and 6
        # -> 6 has 6 segments and 1 that is different from the 1
        # 0, 9 have 6 segments
        cand_09 = []
        for signal in signals:
            if len(signal) == 6:
                if len(number_to_signal[1] - signal) == 0:
                    cand_09.append(signal)
                else:
                    number_to_signal[6] = signal

        # Differentiate between 0 and 9, 4 is completely in set of 9, but in the set of 0
        leftbot_middle_candidates = cand_09[0].symmetric_difference(cand_09[1])
        for candidate in leftbot_middle_candidates:
            if candidate in number_to_signal[4]:
                mid = candidate
                for cand in cand_09:
                    if mid in cand:
                        number_to_signal[9] = cand
            else:
                leftbot = candidate
                for cand in cand_09:
                    if leftbot in cand:
                        number_to_signal[0] = cand

        # Differentiate between 5, 3, and 2
        # -> 5 has not the rightup segment (which we know from 8-6)
        # -> 3 has not the leftbot one (which we know from 0 vs 9)
        # -> 2 is the only one remaining
        rightup = list(number_to_signal[8] - number_to_signal[6])[0]
        for signal in signals:
            if len(signal) == 5:
                if rightup not in signal:
                    number_to_signal[5] = signal
                elif leftbot not in signal:
                    number_to_signal[3] = signal
                else:
                    number_to_signal[2] = signal

        # Convert to another dictionary for easier parsing with output
        signal_to_number = {
            sort_string("".join(signal)): number
            for number, signal in number_to_signal.items()
        }

        # Get output into better format
        output = [sort_string(signal) for signal in output]

        # Calculate the final result
        number = 0
        for signal in output:
            number = number * 10 + signal_to_number[signal]

        numbers.append(number)

    print(f"Solution part 2: {sum(numbers)}")


def main():
    data = get_input_from_file("day8.txt")
    inputs, outputs = [], []

    for line in data:
        inputs.append([sort_string(string) for string in line.split("|")[0].split()])
        outputs.append([sort_string(string) for string in line.split("|")[1].split()])

    part1(inputs, outputs)
    part2(inputs, outputs)


if __name__ == "__main__":
    main()

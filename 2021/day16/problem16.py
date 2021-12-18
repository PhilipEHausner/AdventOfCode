from copy import copy
from functools import reduce
import operator
from typing import List, Tuple

from utils.get_input import get_input_from_file

hex_to_bin = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}


def hex_string_to_binary_string(hex_string: str) -> str:
    return "".join([hex_to_bin[c] for c in hex_string])


class Problem16:
    data: str
    version_numbers: List[int]

    def __init__(self, data: str):
        self.version_numbers = []
        self.data = copy(data)

    def get_next_version_and_type_id(self) -> Tuple[int, str]:
        version = int(self.data[:3], 2)
        type_id = self.data[3:6]
        self.data = self.data[6:]
        self.version_numbers.append(version)
        return version, type_id

    def process_next_package(self) -> int:
        version, type_id = self.get_next_version_and_type_id()
        result = 0
        if type_id == "100":
            result = self.process_literal()
        elif type_id == "000":
            result = self.process_operator("sum")
        elif type_id == "001":
            result = self.process_operator("product")
        elif type_id == "010":
            result = self.process_operator("minimum")
        elif type_id == "011":
            result = self.process_operator("maximum")
        elif type_id == "101":
            result = self.process_operator("greater than")
        elif type_id == "110":
            result = self.process_operator("less than")
        elif type_id == "111":
            result = self.process_operator("equal to")
        return result

    def process_literal(self) -> int:
        literal = ""
        while self.data[0] != "0":
            literal += self.data[1:5]
            self.data = self.data[5:]
        literal += self.data[1:5]
        self.data = self.data[5:]
        return int(literal, 2)

    def process_operator(self, type_operator: str) -> int:
        result = 0
        sub_results = []
        if self.data[0] == "0":
            bit_length = 15
            subpackages_bit = int(self.data[1: 1 + bit_length], 2)
            self.data = self.data[1 + bit_length:]

            old_len = len(self.data)
            while old_len - len(self.data) < subpackages_bit:
                sub_results.append(self.process_next_package())
        else:
            bit_length = 11
            num_packages = int(self.data[1: 1 + bit_length], 2)
            self.data = self.data[1 + bit_length:]

            for _ in range(num_packages):
                sub_results.append(self.process_next_package())

        if type_operator == "sum":
            result = sum(sub_results)
        elif type_operator == "product":
            result = reduce(operator.mul, sub_results, 1)
        elif type_operator == "minimum":
            result = min(sub_results)
        elif type_operator == "maximum":
            result = max(sub_results)
        elif type_operator == "greater than":
            if sub_results[0] > sub_results[1]:
                result = 1
            else:
                result = 0
        elif type_operator == "less than":
            if sub_results[0] < sub_results[1]:
                result = 1
            else:
                result = 0
        elif type_operator == "equal to":
            if sub_results[0] == sub_results[1]:
                result = 1
            else:
                result = 0

        return result

    def sum_version_number(self) -> int:
        self.process_next_package()
        return sum(self.version_numbers)

    def get_result(self) -> int:
        return self.process_next_package()


def main() -> None:
    data = get_input_from_file("day16.txt")
    data = [hex_string_to_binary_string(hex_string) for hex_string in data]

    version_sums = []
    for line in data:
        problem = Problem16(line)
        version_sum = problem.sum_version_number()
        version_sums.append(version_sum)
    print(f"Solution part 1: {sum(version_sums)}")

    results = []
    for line in data:
        problem = Problem16(line)
        results.append(problem.get_result())
    print(f"Solution part 2: {results[-1]}")


if __name__ == "__main__":
    main()

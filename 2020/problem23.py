from __future__ import annotations

import timeit
from typing import Dict, Set

from get_input import get_input_from_file


class Cup:
    def __init__(self, value, prev_cup=None, next_cup=None):
        self._value = value
        self.prev_cup = prev_cup
        self.next_cup = next_cup

    def __repr__(self):
        return f"Cup({self.value})"

    @property
    def value(self):
        return self._value

    @property
    def prev_cup(self):
        return self._prev_cup

    @prev_cup.setter
    def prev_cup(self, cup: Cup):
        self._prev_cup = cup

    @property
    def next_cup(self):
        return self._next_cup

    @next_cup.setter
    def next_cup(self, cup: Cup):
        self._next_cup = cup


class CircularCups:
    cups: list
    cup_hashmap: Dict[int, Cup]
    curr_cup: Cup

    def __init__(self, values: list):
        self.cups = [Cup(values[i]) if i < len(values) else Cup(i+1) for i in range(1000000)]
        self.cup_hashmap = {cup.value: cup for cup in self.cups}
        self.curr_cup = self.cups[0]
        for i, cup in enumerate(self.cups):
            if i == 0:
                cup.prev_cup = self.cups[-1]
                cup.next_cup = self.cups[1]
            elif i == len(self.cups) - 1:
                cup.prev_cup = self.cups[-2]
                cup.next_cup = self.cups[0]
            else:
                cup.prev_cup = self.cups[i-1]
                cup.next_cup = self.cups[i+1]
        self.min_val = min([cup.value for cup in self.cups])
        self.max_val = max([cup.value for cup in self.cups])

    def move(self, iterations: int = None) -> None:
        if iterations:
            while iterations:
                self.move()
                iterations -= 1
            self.sort_cups()
        else:
            moved_cup = self.curr_cup.next_cup
            self.curr_cup.next_cup = self.curr_cup.next_cup.next_cup.next_cup.next_cup
            destination_cup = self._get_dest_cup_for_val(self.curr_cup.value - 1, {moved_cup.value,
                                                                                   moved_cup.next_cup.value,
                                                                                   moved_cup.next_cup.next_cup.value})

            destination_cup.next_cup.prev_cup = moved_cup.next_cup.next_cup
            moved_cup.next_cup.next_cup.next_cup = destination_cup.next_cup
            moved_cup.prev_cup = destination_cup
            destination_cup.next_cup = moved_cup

            self.curr_cup = self.curr_cup.next_cup

    def sort_cups(self) -> None:
        start_cup, cup = self.curr_cup, self.curr_cup.next_cup
        self.cups = [start_cup]
        while start_cup != cup:
            self.cups.append(cup)
            cup = cup.next_cup

    def get_result_problem_1(self) -> str:
        cup1 = self.cup_hashmap[1]
        cup = cup1.next_cup
        res = ""
        while cup != cup1:
            res += str(cup.value)
            cup = cup.next_cup
        return res

    def get_result_problem2(self) -> int:
        cup1 = self.cup_hashmap[1]
        return cup1.next_cup.value * cup1.next_cup.next_cup.value

    def _get_dest_cup_for_val(self, value: int, invalid_cups: Set[int]) -> Cup:
        while True:
            if value in invalid_cups:
                pass
            elif value < self.min_val:
                value = self.max_val + 1
            elif value in self.cup_hashmap.keys():
                res = self.cup_hashmap[value]
                break
            value -= 1
        return res

    def __repr__(self):
        return "CircularCups([" + ", ".join([str(cup.value) for cup in self.cups]) + "])"

    def __str__(self):
        return "\n".join([f"Values: {'[' + ', '.join([str(cup.value) for cup in self.cups]) + ']'}",
                          f"Current Cup: {self.curr_cup.value}"])


if __name__ == '__main__':
    data = get_input_from_file("problem23.txt")

    data = list(map(int, list(data[0])))

    cups = CircularCups(data)

    start = timeit.default_timer()
    cups.move(10000000)
    end = timeit.default_timer()
    print(end - start, "seconds")
    # print(cups.get_result_problem_1())

    print(cups.get_result_problem2())

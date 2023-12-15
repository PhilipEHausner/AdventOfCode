from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum


class Spring(Enum):
    EMPTY = 1
    DAMAGED = 2
    UNKNOWN = 3

    @classmethod
    def from_char(cls, c: str):
        match c:
            case '.':
                return Spring.EMPTY
            case '#':
                return Spring.DAMAGED
            case '?':
                return Spring.UNKNOWN
            case _:
                raise RuntimeError(f"Unknown Spring character '{c}'.")


class _State(ABC):
    @abstractmethod
    def accept(self, count: int):
        raise NotImplementedError

    @abstractmethod
    def step(self, spring: Spring):
        raise NotImplementedError


class State(_State):
    transition: dict[Spring, State]
    count: int
    id: int
    next_id: int = 1

    def __init__(
            self,
            transition_empty: _State = None,
            transition_damaged: _State = None,
    ):
        self.transition = {
            Spring.EMPTY: self if transition_empty is None else transition_empty,
            Spring.DAMAGED: self if transition_damaged is None else transition_damaged,
        }
        self.count = 0
        self.id = State.next_id
        State.next_id += 1

    def accept(self, count: int):
        self.count += count

    def step(self, spring: Spring):
        count = self.count
        self.count = 0
        match spring:
            case Spring.EMPTY:
                self.transition[Spring.EMPTY].accept(count)
            case Spring.DAMAGED:
                self.transition[Spring.DAMAGED].accept(count)
            case Spring.UNKNOWN:
                self.transition[Spring.EMPTY].accept(count)
                self.transition[Spring.DAMAGED].accept(count)

    def __repr__(self) -> str:
        return f"State_{self.id}"


class LostState(_State):

    def accept(self, count: int):
        pass

    def step(self, spring: Spring):
        pass

    def __repr__(self) -> str:
        return "LOST"


class StateMachine:
    lost_state: LostState
    states: list[State]

    def __init__(self, record: list[int]):
        self.lost_state = LostState()
        self.states = []
        last = None
        for entry in reversed(record):
            last = State(transition_damaged=self.lost_state, transition_empty=last)
            self.states.append(last)
            for i in range(entry):
                transition_empty = None if i == entry - 1 else self.lost_state
                last = State(transition_damaged=last, transition_empty=transition_empty)
                self.states.append(last)
        self.states.reverse()
        self.states[0].count = 1

    def run(self, springs: list[Spring]) -> int:
        for spring in springs:
            for state in reversed(self.states):
                state.step(spring)
        return self.states[-1].count


def parse_springs(s: str) -> list[Spring]:
    return [Spring.from_char(c) for c in s]


def parse_record(s: str) -> list[int]:
    return [int(c) for c in s.split(",")]


def solve(line: str, multiplier: int):
    springs, record = line.split()
    springs = "?".join([springs for _ in range(multiplier)])
    record = ",".join([record for _ in range(multiplier)])
    springs = parse_springs(springs)
    record = parse_record(record)
    state_machine = StateMachine(record)
    res = state_machine.run(springs)
    return res


def solve1(lines: list[str]) -> int:
    return sum([solve(line, 1) for line in lines])


def solve2(lines: list[str]) -> int:
    return sum([solve(line, 5) for line in lines])


def main(file: str):
    with open(file, "r") as f:
        lines = f.readlines()

    print(f"Result problem 1: {solve1(lines)}")
    print(f"Result problem 2: {solve2(lines)}")


if __name__ == "__main__":
    main("./files/day12.txt")

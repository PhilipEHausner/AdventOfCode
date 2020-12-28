import collections
from typing import DefaultDict, List, Tuple

from get_input import get_input_from_file


def take_step(command: str) -> Tuple[int, int]:
    res = (0, 0)
    if command == "e":
        res = (0, -1)
    elif command == "se":
        res = (-1, 0)
    elif command == "sw":
        res = (-1, 1)
    elif command == "w":
        res = (0, 1)
    elif command == "nw":
        res = (1, 0)
    elif command == "ne":
        res = (1, -1)
    return res


def find_tile(command: str) -> Tuple[int, int]:
    i = 0
    way = (0, 0)
    while i < len(command):
        if command[i] == "s" or command[i] == "n":
            step = take_step(command[i:i + 2])
            i += 1
        else:
            step = take_step(command[i])
        way = (way[0] + step[0], way[1] + step[1])

        i += 1
    return way


def turn_tiles(_tiles: DefaultDict[Tuple[int, int], int], commands: List[str]) -> None:
    for command in commands:
        way = find_tile(command)

        _tiles[way] += 1

    for tile in _tiles.keys():
        _tiles[tile] = _tiles[tile] % 2


def apply_rule(_tiles: DefaultDict[Tuple[int, int], int]) -> DefaultDict[Tuple[int, int], int]:
    counts = collections.defaultdict(int)

    steps = [(0, -1), (-1, 0), (-1, 1), (0, 1), (1, 0), (1, -1)]

    for tile, color in _tiles.items():
        for step in steps:
            if color == 1:
                counts[(tile[0] + step[0], tile[1] + step[1])] += 1

    new_tiles = collections.defaultdict(int)
    keys = set(_tiles.keys()).union(counts.keys())

    for key in keys:
        # white tile
        if _tiles[key] == 0 and counts[key] == 2:
            new_tiles[key] = 1
        elif _tiles[key] == 1 and (counts[key] == 0 or counts[key] > 2):
            new_tiles[key] = 0
        else:
            new_tiles[key] = _tiles[key]

    return new_tiles


if __name__ == '__main__':
    data = get_input_from_file("problem24.txt")

    tiles = collections.defaultdict(int)

    turn_tiles(tiles, data)

    print("Problem 1:", sum([i for i in tiles.values()]))

    for day in range(100):
        tiles = apply_rule(tiles)
        print(f"Problem 2 (day {day+1}):", sum([i for i in tiles.values()]))

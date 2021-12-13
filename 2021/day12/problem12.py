import collections
from typing import Dict, List, Set

from utils.get_input import get_input_from_file


def get_path(edges: Dict[str, List], visited: Set, current_vertex: str) -> int:
    if current_vertex == "end":
        return 1

    candidates = [vertex for vertex in edges[current_vertex] if vertex not in visited]

    num_paths = 0
    for candidate in candidates:
        new_visited = visited.copy()
        if current_vertex.lower() == current_vertex:
            new_visited.add(current_vertex)
        num_paths += get_path(edges, new_visited, candidate)

    return num_paths


def get_path_part_2(edges: Dict[str, List], visited: List, current_vertex: str) -> int:
    if current_vertex == "end":
        return 1

    counter = collections.Counter(visited)
    counter["start"] = 0
    for k, v in counter.items():
        if k.upper() == k:
            counter[k] = 0
    if any(x > 1 for x in counter.values()):
        candidates = [vertex for vertex in edges[current_vertex] if
                      (vertex not in visited and vertex != "start") or vertex.upper() == vertex]
    else:
        candidates = [vertex for vertex in edges[current_vertex] if vertex != "start" or vertex.upper() == vertex]

    num_paths = 0
    for candidate in candidates:
        new_visited = visited.copy()
        new_visited.append(candidate)
        num_paths += (get_path_part_2(edges, new_visited, candidate))

    return num_paths


def part1(edges: Dict[str, List]) -> None:
    print(f"Solution part 1: {get_path(edges, {'start'}, 'start')}")


def part2(edges: Dict[str, List]) -> None:
    paths = get_path_part_2(edges, ['start'], 'start')
    print(f"Solution part 2: {paths}")


def main() -> None:
    data = get_input_from_file("day12.txt")
    edges = collections.defaultdict(list)

    for line in data:
        edge = line.split("-")
        edges[edge[0]].append(edge[1])
        edges[edge[1]].append(edge[0])

    part1(edges)
    part2(edges)


if __name__ == "__main__":
    main()

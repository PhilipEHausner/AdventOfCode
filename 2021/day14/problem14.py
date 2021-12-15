import collections
from typing import Dict

from utils.get_input import get_input_from_file


def part1(template: str, rules: Dict[str, str], num_steps: int) -> int:
    # Translate rules pair -> insert to mappings pair -> (new_pair, new_pair)
    mappings = {
        pair: (pair[0] + insert, insert + pair[1])
        for pair, insert in rules.items()
    }

    # Count the pairs that are initially present
    polymer_pairs = collections.defaultdict(int)
    for i in range(0, len(template)-1):
        polymer_pairs[template[i:i+2]] += 1

    # For each step transform all pairs into their new pairs (using mapping) and count the new pairs
    for _ in range(num_steps):
        new_polymer_pairs = collections.defaultdict(int)
        for pair in polymer_pairs.keys():
            if pair in mappings.keys():
                new_pairs = mappings[pair]
                new_polymer_pairs[new_pairs[0]] += polymer_pairs[pair]
                new_polymer_pairs[new_pairs[1]] += polymer_pairs[pair]
            else:
                new_polymer_pairs[pair] += polymer_pairs[pair]
        polymer_pairs = new_polymer_pairs

    # In the end, we do not want to count pairs but single elements
    # Divide by 2, since every character appears in two pair representations, e.g., for "BCB" both "BC" and "CB"
    # are present in the polymer_pairs, but C is only present once.
    single_element_counter = collections.defaultdict(int)
    for pair, count in polymer_pairs.items():
        single_element_counter[pair[0]] += count
        single_element_counter[pair[1]] += count
    single_element_counter = {
        key: value // 2
        for key, value in single_element_counter.items()
    }

    # Last and first element of template are only counted once, since they only appear in one tuple.
    single_element_counter[template[0]] += 1
    single_element_counter[template[-1]] += 1

    return max(single_element_counter.values()) - min(single_element_counter.values())


def main() -> None:
    data = get_input_from_file("day14.txt")
    template = data[0]

    rules = {}
    for line in data[2:]:
        pair, insert = line.split("->")
        pair = pair.strip()
        insert = insert.strip()
        rules[pair] = insert

    print(f"Solution part 1: {part1(template, rules, 10)}")
    print(f"Solution part 2: {part1(template, rules, 40)}")


if __name__ == "__main__":
    main()

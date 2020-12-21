import collections
from functools import reduce

from get_input import get_input_from_file


if __name__ == '__main__':
    data = get_input_from_file("problem16.txt")

    data = collections.deque([d for d in data if d])
    valid_ranges = collections.defaultdict(list)
    key_indices = {}
    index_to_key = {}

    current_line = data.popleft()
    curr_index = 0

    while not current_line.lower().startswith("your ticket"):
        key, ranges = current_line.split(":")
        ranges = [r.strip() for r in ranges.split("or")]
        ranges = [(int(r[0]), int(r[1])) for r in [r.split("-") for r in ranges]]
        valid_ranges[key] = ranges
        key_indices[key] = curr_index
        index_to_key[curr_index] = key
        current_line = data.popleft()
        curr_index += 1

    current_line = data.popleft()
    my_ticket = []

    while not current_line.lower().startswith("nearby tickets"):
        my_ticket = [int(val.strip()) for val in current_line.split(",")]
        current_line = data.popleft()

    all_valid_ranges = [val for subl in valid_ranges.values() for val in subl]
    tickets = []

    while len(data):
        current_line = data.popleft()
        if current_line:
            values = [int(val.strip()) for val in current_line.split(",")]
            is_ticket = True
            for val in values:
                if not any([v[0] <= val <= v[1] for v in all_valid_ranges]):
                    is_ticket = False
                    break
            if is_ticket:
                tickets.append(values)

    valid_fields = {}

    for category, ranges in valid_ranges.items():
        possible_fields = set(range(len(tickets[0])))
        for ticket in tickets:
            for i, val in enumerate(ticket):
                possible = False
                for r in ranges:
                    if r[0] <= val <= r[1]:
                        possible = True
                        break
                if not possible:
                    possible_fields.remove(i)
        valid_fields[category] = possible_fields

    index_to_category = {}
    while len(index_to_category.keys()) != len(valid_fields.keys()):
        for category, valids in valid_fields.items():
            if len(valids) == 1:
                val = valids.pop()
                index_to_category[val] = category
                for cat, valids2 in valid_fields.items():
                    if val in valids2:
                        valids2.remove(val)

    my_values = []
    for ind, cat in index_to_category.items():
        if cat.startswith("departure"):
            my_values.append(my_ticket[ind])

    print(my_values)
    print(reduce(lambda x, y: x * y, my_values, 1))

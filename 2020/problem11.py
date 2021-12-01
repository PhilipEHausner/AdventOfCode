import copy

from get_input import get_input_from_file


def num_of_occupied_adjacent_seats(row, col, array):
    row_above = row - 1
    row_below = row + 1
    col_left = col - 1
    col_right = col + 1
    height = len(array)
    width = len(array[0])

    fields_to_check = [
        (row_above, col_left),
        (row_above, col),
        (row_above, col_right),
        (row, col_left),
        (row, col_right),
        (row_below, col_left),
        (row_below, col),
        (row_below, col_right),
    ]
    fields_to_check = [field for field in fields_to_check
                       if 0 <= field[0] < height and 0 <= field[1] < width]

    count = sum([1 for field in fields_to_check if array[field[0]][field[1]] == "#"])
    return count


def num_of_occupied_adjacent_seats_ignore_floor(row, col, array):
    res = 0
    height = len(array)
    width = len(array[0])
    directions = [(+1, 0), (-1, 0), (0, +1), (0, -1), (-1, -1), (+1, -1), (+1, +1), (-1, +1)]
    for d in directions:
        curr_row, curr_col = row, col
        curr_row += d[0]
        curr_col += d[1]
        while 0 <= curr_row < height and 0 <= curr_col < width:
            if array[curr_row][curr_col] == "#":
                res += 1
                break
            elif array[curr_row][curr_col] == "L":
                break
            curr_row += d[0]
            curr_col += d[1]
    return res


def print_data(array):
    for line in array:
        for el in line:
            print(el, end=" ")
        print()
    print()


def print_counts(county, array):
    for a in range(len(array)):
        for b in range(len(array[a])):
            print(county[(a, b)], end=" ")
        print()
    print()


if __name__ == '__main__':
    data = get_input_from_file("problem11.txt")

    data = [list(line) for line in data]

    buffered = copy.deepcopy(data)

    num_iter = 0
    previous_states = []

    while True:
        print(num_iter)
        num_iter += 1

        for i in range(len(data)):
            for j in range(len(data[i])):
                if data[i][j] == ".":
                    continue
                # occupied_adj = num_of_occupied_adjacent_seats(i, j, data)
                occupied_adj = num_of_occupied_adjacent_seats_ignore_floor(i, j, data)
                if data[i][j] == "L" and occupied_adj == 0:
                    buffered[i][j] = "#"
                if data[i][j] == "#" and occupied_adj >= 5:
                    buffered[i][j] = "L"

        if buffered == data:
            break

        data = copy.deepcopy(buffered)

    # Number of occupied seats
    result = sum([sum([1 for el in line if el == "#"]) for line in data])
    print(f"Number of occupied seats: {result}")

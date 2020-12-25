import collections
from functools import reduce

import numpy as np

from get_input import get_input_from_file

if __name__ == '__main__':
    data = get_input_from_file("problem20.txt")

    tiles = []
    tile_number_to_tile = {}
    tile_number = 0
    tile = []
    for line in data:
        if line.startswith("Tile"):
            line = line.split()[1][:-1]
            tile_number = int(line)

        elif not line:
            tile = np.array([list(t) for t in tile])
            tiles.append((tile_number, tile))
            tile_number_to_tile[tile_number] = tile
            tile = []

        else:
            tile.append(line)

    tile = np.array([list(t) for t in tile])
    tiles.append((tile_number, tile))
    tile_number_to_tile[tile_number] = tile

    matches = []

    for i in range(len(tiles)):
        tile1 = tiles[i][1]
        for j in range(len(tiles)):
            if i == j:
                continue
            tile2 = tiles[j][1]
            for k1 in range(4):
                curr_tile1 = np.rot90(tile1, k1)
                for k2 in range(4):
                    curr_tile2 = np.rot90(tile2, k2)
                    if all(curr_tile1[0] == curr_tile2[0]):
                        matches.append((tiles[i][0], tiles[j][0]))  # , k1, k2, False, False))

                    if all(curr_tile1[0] == curr_tile2[0][::-1]):
                        matches.append((tiles[i][0], tiles[j][0]))  # , k1, k2, False, True))

                    # if all(curr_tile1[0][::-1] == curr_tile2[0]):
                    #     matches.append((tiles[i][0], tiles[j][0], k1, k2, True, False))

    new_matches = []
    for i in range(len(matches)):
        found = False
        for j in range(i+1, len(matches)):
            if matches[i][0] == matches[j][1] and matches[i][1] == matches[j][0]:
                found = True

        if not found:
            new_matches.append(matches[i])
    matches = new_matches

    counts = collections.defaultdict(int)
    for match in matches:
        for m in match:
            counts[m] += 1

    corners = []
    for key, count in sorted(counts.items(), key=lambda x: x[1]):
        if count == 2:
            corners.append(key)

    res1 = reduce(lambda x, y: x * y, map(int, corners), 1)

    # TODO: Specify shape
    grid = np.zeros((120, 120), dtype="<U1")
    tile_number_grid = [[0 for _ in range(12)] for _ in range(12)]

    start_corner = corners[0]
    corner = tile_number_to_tile[start_corner]
    tile_number_grid[0][0] = start_corner
    candidates = []
    already_done = {start_corner}
    for match in matches:
        if match[0] == start_corner:
            candidates.append((match[1], tile_number_to_tile[match[1]]))
        if match[1] == start_corner:
            candidates.append((match[0], tile_number_to_tile[match[0]]))

    sol_found = False
    count = 0
    while not sol_found and count < 10:
        count += 1
        corner = np.rot90(corner, 1)
        corner_right_row = corner[:, -1]
        corner_bottom_row = corner[-1, :]

        grid[:10, :10] = corner

        found_right = False
        found_bottom = False
        for tile_num, candidate in candidates:
            already_done.add(tile_num)
            for _ in range(4):
                candidate = np.rot90(candidate, 1)
                cand_left_row = candidate[:, 0]
                cand_top_row = candidate[0, :]

                if not found_right and all(cand_left_row == corner_right_row):
                    tile_number_grid[0][1] = tile_num
                    grid[:10, 10:20] = candidate
                    found_right = True
                    break

                # right row, but inverted
                if not found_right and all(cand_left_row == corner_right_row[::-1]):
                    tile_number_grid[0][1] = tile_num
                    grid[:10, 10:20] = np.flip(candidate, axis=0)
                    found_right = True
                    break

                if not found_bottom and all(cand_top_row == corner_bottom_row):
                    tile_number_grid[1][0] = tile_num
                    grid[10:20, :10] = candidate
                    found_bottom = True
                    break

                if not found_bottom and all(cand_top_row == corner_bottom_row[::-1]):
                    tile_number_grid[1][0] = tile_num
                    grid[10:20, :10] = np.flip(candidate, axis=1)
                    found_bottom = True
                    break

        if found_bottom and found_right:
            sol_found = True
    matched = 3
    curr_row = 0
    curr_column = 1
    first_wrap = False
    breadth = 2

    while matched != len(tiles):
        curr_tile = grid[curr_row*10: curr_row*10+10, curr_column*10: curr_column*10+10]
        curr_tile_num = tile_number_grid[curr_row][curr_column]
        print("curr tile num:", curr_tile_num)

        curr_tile_right_row = curr_tile[:, -1]
        curr_tile_bottom_row = curr_tile[-1, :]

        found_right = False

        candidates = []
        for match in matches:
            if match[0] == curr_tile_num and match[1] not in already_done:
                candidates.append((match[1], tile_number_to_tile[match[1]]))
            if match[1] == curr_tile_num and match[0] not in already_done:
                candidates.append((match[0], tile_number_to_tile[match[0]]))

        found_right = False
        found_bottom = False

        for tile_num, candidate in candidates:
            already_done.add(tile_num)

            for _ in range(4):
                candidate = np.rot90(candidate, 1)
                cand_left_row = candidate[:, 0]
                cand_top_row = candidate[0, :]

                if not found_right and all(cand_left_row == curr_tile_right_row):
                    tile_number_grid[curr_row][curr_column+1] = tile_num
                    grid[curr_row*10: curr_row*10+10, curr_column*10+10: curr_column*10+20] = candidate
                    found_right = True
                    matched += 1
                    if not first_wrap:
                        breadth += 1
                    break

                # right row, but inverted
                if not found_right and all(cand_left_row == curr_tile_right_row[::-1]):
                    tile_number_grid[curr_row][curr_column + 1] = tile_num
                    grid[curr_row * 10: curr_row * 10 + 10, curr_column * 10 + 10: curr_column * 10 + 20] \
                        = np.flip(candidate, axis=0)
                    found_right = True
                    matched += 1
                    if not first_wrap:
                        breadth += 1
                    break

                if not found_bottom and all(cand_top_row == curr_tile_bottom_row):
                    tile_number_grid[curr_row+1][curr_column] = tile_num
                    grid[curr_row*10+10: curr_row*10+20, curr_column*10: curr_column*10+10] = candidate
                    found_bottom = True
                    matched += 1
                    break

                # bottom row, but inverted
                if not found_bottom and all(cand_top_row == curr_tile_bottom_row[::-1]):
                    tile_number_grid[curr_row+1][curr_column] = tile_num
                    grid[curr_row*10+10: curr_row*10+20, curr_column*10: curr_column*10+10] = np.flip(candidate, axis=1)
                    found_bottom = True
                    matched += 1
                    break

        if not first_wrap:
            curr_column += 1
            if not found_right:
                curr_column = 0
                curr_row += 1
                first_wrap = True
        else:
            curr_column += 1
            if curr_column == breadth:
                curr_column = 0
                curr_row += 1

    final_grid = np.zeros((96, 96), dtype="<U1")

    for row in range(12):
        for col in range(12):
            final_grid[8*row:8*row+8, 8*col:8*col+8] = grid[10*row+1:10*row+9, 10*col+1:10*col+9]


    def check_for_pattern(g, pat):
        has_pattern = True
        for r in range(g.shape[0]):
            for c in range(g.shape[1]):
                if g[r][c] != "#" and pat[r][c] == "#":
                    has_pattern = False

        return has_pattern

    pattern = np.array(
        [[".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "#", "."],
         ["#", ".", ".", ".", ".", "#", "#", ".", ".", ".", ".", "#", "#", ".", ".", ".", ".", "#", "#", "#"],
         [".", "#", ".", ".", "#", ".", ".", "#", ".", ".", "#", ".", ".", "#", ".", ".", "#", ".", ".", "."]])
    replacement = np.array(
        [[".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "O", "."],
         ["O", ".", ".", ".", ".", "O", "O", ".", ".", ".", ".", "O", "O", ".", ".", ".", ".", "O", "O", "O"],
         [".", "O", ".", ".", "O", ".", ".", "O", ".", ".", "O", ".", ".", "O", ".", ".", "O", ".", ".", "."]])

    for _ in range(4):
        final_grid = np.rot90(final_grid, 1)
        for _ in range(2):
            final_grid = np.flip(final_grid, axis=0)
            for _ in range(2):
                final_grid = np.flip(final_grid, axis=1)
                for row in range(final_grid.shape[0] - pattern.shape[0] + 1):
                    for col in range(final_grid.shape[1] - pattern.shape[1] + 1):
                        curr_snippet = final_grid[row:row+pattern.shape[0], col:col+pattern.shape[1]]
                        if check_for_pattern(curr_snippet, pattern):
                            for x in range(pattern.shape[0]):
                                for y in range(pattern.shape[1]):
                                    if pattern[x][y] == "#":
                                        final_grid[row+x][col+y] = "O"

    count = 0
    for row in final_grid:
        for el in row:
            if el == "#":
                count += 1
    print(count)


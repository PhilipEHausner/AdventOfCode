import collections
from functools import reduce

import numpy as np

from get_input import get_input_from_file

if __name__ == '__main__':
    data = get_input_from_file("problem20.txt")

    tiles = []
    tile_number = 0
    tile = []
    for line in data:
        if line.startswith("Tile"):
            line = line.split()[1][:-1]
            tile_number = int(line)

        elif not line:
            tile = np.array([list(t) for t in tile])
            tiles.append((tile_number, tile))
            tile = []

        else:
            tile.append(line)

    tile = np.array([list(t) for t in tile])
    tiles.append((tile_number, tile))

    grid = tiles[0][1]

    for i in range(len(tiles)):
        tile = tiles[i][1]

    # for i in range(len(tiles)):
    #     tile1 = tiles[i][1]
    #     for j in range(len(tiles)):
    #         if i == j:
    #             continue
    #         tile2 = tiles[j][1]
    #         for k1 in range(4):
    #             curr_tile1 = np.rot90(tile1, k1)
    #             for k2 in range(4):
    #                 curr_tile2 = np.rot90(tile2, k2)
    #                 if all(curr_tile1[0] == curr_tile2[0]):
    #                     matches.append((tiles[i][0], tiles[j][0], k1, k2, False, False))
    #
    #                 if all(curr_tile1[0] == curr_tile2[0][::-1]):
    #                     matches.append((tiles[i][0], tiles[j][0], k1, k2, False, True))
    #
    #                 if all(curr_tile1[0][::-1] == curr_tile2[0]):
    #                     matches.append((tiles[i][0], tiles[j][0], k1, k2, True, False))


    # new_matches = []
    # for i in range(len(matches)):
    #     found = False
    #     for j in range(i+1, len(matches)):
    #         if matches[i][0] == matches[j][1] and matches[i][1] == matches[j][0]:
    #             found = True
    #
    #     if not found:
    #         new_matches.append(matches[i])
    # matches = new_matches

    # counts = collections.defaultdict(int)
    # for match in matches:
    #     for m in match:
    #         counts[m] += 1
    #
    # corners = []
    # for key, count in sorted(counts.items(), key=lambda x: x[1]):
    #     print(key, count)
    #     if count == 4:
    #         corners.append(key)
    #
    # res = reduce(lambda x, y: x * y, map(int, corners), 1)
    # print(res)

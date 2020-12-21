import numpy as np
from scipy.signal import convolve

from get_input import get_input_from_file


def perform_cycle(state, dims):
    # Expand data
    state = np.pad(state, 1)
    neighbours = convolve(state, np.ones(tuple(3 for _ in range(dims)), dtype=int), "same") - state

    remain_active = (neighbours == 2) | (neighbours == 3)
    get_active = neighbours == 3

    new_state = (state == 1) & remain_active
    new_state = new_state | get_active

    new_state = np.array(new_state, dtype=int)

    return new_state


if __name__ == '__main__':
    data = get_input_from_file("problem17.txt")

    dimensions = 4
    num_cycles = 6

    data = [list(d) for d in data]
    for i, dat in enumerate(data):
        data[i] = [0 if d == "." else 1 for d in dat]
    data = np.array(data)
    shape = [1 for _ in range(dimensions)]
    shape[-2], shape[-1] = data.shape[0], data.shape[1]
    data.shape = shape

    for _ in range(num_cycles):
        data = perform_cycle(data, dimensions)

    print(np.sum(data))

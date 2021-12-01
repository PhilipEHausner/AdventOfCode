import collections
from functools import lru_cache

from get_input import get_input_from_file


class DataStorage:
    instance = None

    def __init__(self, arr=None):
        if not DataStorage.instance or arr:
            DataStorage.instance = arr
        self.arr = DataStorage.instance

    def __call__(self, *args, **kwargs):
        return self.arr


@lru_cache(maxsize=200)
def get_number_of_possibilities(index: int):
    arr = DataStorage()()
    if index == len(arr) - 1:
        return 1
    res = 0
    res += get_number_of_possibilities(index+1)
    if index <= len(arr) - 3 and arr[index] + 3 >= arr[index+2]:
        res += get_number_of_possibilities(index+2)
    if index <= len(arr) - 4 and arr[index] + 3 >= arr[index+3]:
        res += get_number_of_possibilities(index+3)
    return res


if __name__ == '__main__':
    data = get_input_from_file("problem10.txt")
    data = map(int, data)
    data = sorted(data)

    ones = 0
    twos = 0
    threes = 1

    if data[0] == 1:
        ones += 1
    elif data[0] == 2:
        twos += 1
    elif data[0] == 3:
        threes += 1

    for i in range(len(data)-1):
        if data[i] + 1 == data[i+1]:
            ones += 1
        elif data[i] + 2 == data[i+1]:
            twos += 1
        elif data[i] + 3 == data[i+1]:
            threes += 1
        else:
            print("ALARM")
    data = [0] + data
    DataStorage(data)
    possibilities = get_number_of_possibilities(0)
    print(possibilities)

    possibilities = collections.defaultdict(int)
    possibilities[len(data) - 1] = 1

    curr_index = len(data) - 1
    while curr_index > 0:
        if curr_index >= 3:
            if data[curr_index-3] + 3 >= data[curr_index]:
                possibilities[curr_index-3] += possibilities[curr_index]
        if curr_index >= 2:
            if data[curr_index-2] + 3 >= data[curr_index]:
                possibilities[curr_index-2] += possibilities[curr_index]
        if curr_index >= 1:
            if data[curr_index-1] + 3 >= data[curr_index]:
                possibilities[curr_index-1] += possibilities[curr_index]
        curr_index -= 1
    print(possibilities[0])

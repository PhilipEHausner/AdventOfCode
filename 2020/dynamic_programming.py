import collections
import functools


@functools.lru_cache(maxsize=10000)
def fibonacci(n):
    if n == 1:
        return 1
    if n == 0:
        return 1
    return fibonacci(n-1) + fibonacci(n-2)


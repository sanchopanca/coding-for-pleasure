import collections
import itertools


def sliding_window(iterable, n):
    # sliding_window('ABCDEFG', 4) -> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(itertools.islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window)


with open('../input/01.txt') as f:
    depth = list(map(int, f.readlines()))

print(sum([1 for a, b in sliding_window(depth, 2) if b > a]))
print(sum([1 for a, b in sliding_window(
    [sum(d) for d in sliding_window(depth, 3)],
    2) if b > a]))


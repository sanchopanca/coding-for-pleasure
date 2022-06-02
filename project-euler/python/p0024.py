from itertools import permutations

print(''.join(map(str, list(permutations(range(10)))[10**6-1])))

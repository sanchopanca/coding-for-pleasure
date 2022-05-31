def concatenated_product(x, n):
    return ''.join([str(x * i) for i in range(1, n+1)])


def is_1_9_pandigital(s):
    return len(s) == len(set(s)) == 9 and '0' not in s


result = 0

for x in range(1, 10_000):
    for n in range(10):
        if is_1_9_pandigital(concatenated_product(x, n)):
            result = max(x, int(concatenated_product(x, n)))

print(result)

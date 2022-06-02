def s5(n):
    return sum([d ** 5 for d in map(int, str(n))])


print(sum([x for x in range(2, 1_000_000) if x == s5(x)]))

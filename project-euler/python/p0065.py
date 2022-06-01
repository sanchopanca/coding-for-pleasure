from fractions import Fraction


def denominators_e(n):
    k = 1
    res = []
    while len(res) < n:
        res.append(1)
        res.append(2 * k)
        res.append(1)
        k += 1
    return list(reversed(res[:n]))


n = Fraction(0)

for d in denominators_e(99):
    n = Fraction(1) / (d + n)

n += 2

print(sum(map(int, str(n.numerator))))

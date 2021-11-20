from fractions import Fraction


def main():
    t = int(input())
    for _ in range(t):
        u, v = map(int, input().split())
        f = (Fraction(1, u + v) - Fraction(1, v)) / (Fraction(1, u) - Fraction(1, u + v))
        # print(u, v)
        print(f.numerator, f.denominator)

main()

def am_deviation(a1, a2, a3):
    x = abs(a1 + a3 - 2 * a2)
    return x

def op1(a1, a2, a3):
    return a1 - 1, a2 + 1, a3

def op2(a1, a2, a3):
    return a1 + 1, a2 - 1, a3

def main():
    t = int(input())
    for _ in range(t):
        a1, a2, a3 = sorted(map(int, input().split()))
        amd = am_deviation(a1, a2, a3)
        if amd % 3 == 0:
            print(0)
        else:
            print(1)


main()

# a1, a2, a3 = -10, 3, 19
# for _ in range(10):
#     print(am_deviation(a1, a2, a3))
#     a1, a2, a3 = op1(a1, a2, a3)
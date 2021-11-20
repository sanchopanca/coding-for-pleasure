from math import floor, pow, sqrt

def main():
    n = int(input())
    # n = 4
    result = 0
    for a in range(1, floor(pow(n, 1/3)) + 1):
        for b in range(a, floor(sqrt(n/a)) + 1):
            result += floor(n/(a*b)) - b + 1
    print(result)

main()

def S(a, b):
    return 4 * a * b + 3 * a + 3 * b

def main():
    n = int(input())
    s = map(int, input().split())
    p = set()
    for a in range(1, 150):
        for b in range(1, 150):
            p.add(S(a, b))
    result = 0
    for i in s:
        if i not in p:
            result += 1
    print(result)


main()

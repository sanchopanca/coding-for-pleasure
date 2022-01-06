from collections import Counter


def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        a = Counter(map(int, input().split()))
        second_team = max(a.values())
        first_team = len(a) - 1
        if second_team > first_team:
            print(min(first_team + 1, second_team - 1))
        else:
            print(second_team)


main()

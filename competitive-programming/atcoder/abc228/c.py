def main():
    n, k = map(int, input().split())
    scores = []
    for _ in range(n):
        scores.append(sum(map(int, input().split())))
    sorted_scores = sorted(scores)
    for score in scores:
        if sorted_scores[-k] - score <= 300:
            print("Yes")
        else:
            print("No")


main()

def main():
    n, k = map(int, input().split())
    # A = sorted(map(int, input().split()), reverse=True)
    A = sorted(map(int, input().split()))
    res = 0
    diffs = []
    for i in range(1, n):
        diffs.append(A[i] - A[i - 1])
    res += A[0]
    for i in range(1, n):
        pass



main()

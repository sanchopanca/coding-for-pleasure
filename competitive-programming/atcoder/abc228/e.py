def pow_mod(x, n, m):
    y = 1
    while n > 0:
        if n % 2 == 1:
            y = y * x % m
        n = n // 2
        x = x * x % m
    return y


# def pow(x, n):
#     y = 1
#     while n > 0:
#         if n % 2 == 1:
#             y = y * x
#         n = n // 2
#         x = x * x
#     return y


def main():
    n, k, m = map(int, input().split())
    # if m % 998244353 == 0:
    #     print(0)
    #     return
    n_seq = pow_mod(k, n, 998244352)
    n_scores = pow_mod(m, n_seq, 998244353)
    print(n_scores)


main()

# print(pow_mod(11, 21321321, 11))
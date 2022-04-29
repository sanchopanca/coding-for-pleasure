def is_palindrome(n: int):
    return list(str(n)) == list(reversed(str(n)))

res = 0

for i in range(100, 1000):
    for j in range(100, 1000):
        if (is_palindrome(i * j)):
            res = max(res, i * j)

print(res)

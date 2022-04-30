all_digits = set("123456789")
res = set()
for i in range(1, 9_999_999):
    for j in range(i + 1, 9_999_999 // i):
        s = str(i * j) + str(i) + str(j)
        if len(s) > 9:
            break
        if len(s) == 9 and set(s) == all_digits:
            res.add(i * j)
print(sum(res))

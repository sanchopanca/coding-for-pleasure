import string

n = int(input())

all_lowercase = set(string.ascii_lowercase)

for _ in range(n):
    s = set(input().strip().lower())
    s = s & all_lowercase
    missing = all_lowercase - s
    if missing:
        print('missing', ''.join(sorted(missing)))
    else:
        print('pangram')

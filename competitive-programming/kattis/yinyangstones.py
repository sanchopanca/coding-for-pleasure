from collections import Counter

c = Counter(input().strip())
print(int(c['W'] == c['B']))

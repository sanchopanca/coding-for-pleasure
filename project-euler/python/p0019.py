from datetime import date, timedelta

d = date(1901, 1, 6)  # first Sunday
end = date(2000, 12, 31)

res = 0

while d <= end:
    if d.day == 1:
        res += 1
    d += timedelta(days=7)

print(res)

m = {
    'M': 1000,
    'CM': 900,
    'D': 500,
    'CD': 400,
    'C': 100,
    'XC': 90,
    'L': 50,
    'XL': 40,
    'X': 10,
    'IX': 9,
    'V': 5,
    'IV': 4,
    'I': 1,
}


def roman_to_int(r):
    result = 0
    i = 0
    while i < len(r):
        if i + 1 < len(r) and r[i:i+2] in m:
            result += m[r[i:i+2]]
            i += 2
            continue
        result += m[r[i]]
        i += 1
    return result


def int_to_roman(n):
    result = ''
    for roman, arabic in m.items():  # order is guaranteed
        while n >= arabic:
            result += roman
            n -= arabic
    return result


space_saved = 0

with open('../input/p089_roman.txt') as f:
    for line in f:
        r = line.strip()
        space_saved += len(r) - len(int_to_roman(roman_to_int(r)))
print(space_saved)

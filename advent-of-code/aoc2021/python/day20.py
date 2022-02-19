from collections import defaultdict


def enhance(img, X, Y, enhancement):
    new_img = defaultdict(bool)
    for y in range(-10, Y+10):
        for x in range(-10, X+10):
            b = ''
            for dx in range(-1, 2):
                for dy in range(-1, 2):
                    if img[(x+dx, y+dy)]:
                        b += '1'
                    else:
                        b += '0'
            b = int(b, 2)
            new_img[(x, y)] = enhancement[b] == '#'
    return new_img


def part1():
    with open('../input/20s.txt', 'r') as f:
        enhancement, image_str = f.read().split('\n\n')
    image_str.strip()
    image = defaultdict(bool)
    image_str = image_str.split('\n')
    Y, X = len(image_str), len(image_str[0])
    for y, line in enumerate(image_str):
        for x, c in enumerate(line):
            image[(x, y)] = c == '#'

    for _ in range(1):
        image = enhance(image, X, Y, enhancement)
    s = 0
    for v in image.values():
        if v:
            s += 1
    print(s)



part1()

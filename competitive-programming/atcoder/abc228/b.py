def main():
    n, x = map(int, input().split())
    a = [-1]
    a.extend(map(int, input().split()))
    know = set()
    queue = [x]
    while len(queue) > 0:
        f = queue.pop()
        if f in know:
            continue
        know.add(f)
        queue.append(a[f])

    print(len(know))

main()

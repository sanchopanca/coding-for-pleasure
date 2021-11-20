def main():
    s, t, x = map(int, input().split())
    if t > s:
        print('Yes') if s <= x < t else print('No')
    else:
        print('Yes') if x >= s or x < t else print('No')


main()

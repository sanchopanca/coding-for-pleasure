def main():
    s1 = input().strip()
    s2 = input().strip()

    b = 0
    b += s1.count('#')
    b += s2.count('#')
    if b > 2:
        print('Yes')
        return
    if s1.count('#') == 2 or s2.count('#') == 2:
        print('Yes')
        return
    if s1.index('#') == s2.index('#'):
        print('Yes')
        return
    print('No')


main()

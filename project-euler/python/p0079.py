potential_symbols = set()

logins = []
with open('../input/p079_keylog.txt') as f:
    for line in f:
        logins.append(line.strip())
        potential_symbols.update(line.strip())

min_len = len(potential_symbols)

for password in range(10 ** (min_len - 1), 10 ** (min_len + 3)):
    password = str(password)
    for l in logins:
        if l[0] in password and l[1] in password and l[2] in password and \
                (password.index(l[0]) < password.index(l[1]) < password.index(l[2])):
            pass
        else:
            break
    else:
        print(password)
        break

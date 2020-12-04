with open("input.txt") as fin:
    lines = fin.read().split('\n')
    lines.pop()
    rules = [(1,1), (3,1), (5,1), (7,1), (1,2)]
    multiplied = 1

    for rule in rules:
        trees = 0
        x = 0
        for y in range(0, len(lines), rule[1]):
            chars = list(lines[y])
            if x >= len(chars):
                x -= len(chars)
            if chars[x] == '#':
                chars[x] = 'X'
                trees += 1
            else:
                chars[x] = 'O'

            x += rule[0]

        print("Trees encountered:", trees)
        multiplied *= trees

    print("Multiplied:", multiplied)

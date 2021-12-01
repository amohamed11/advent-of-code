with open("input.txt") as fin:
    lines = fin.read().split('\n')
    lines.pop()
    trees = 0
    x = 0
    for y in range(len(lines)):
        chars = list(lines[y])
        if x >= len(chars):
            x -= len(chars)
        if chars[x] == '#':
            chars[x] = 'X'
            trees += 1
        else:
            chars[x] = 'O'

        x += 3

    print("Trees encountered:", trees)


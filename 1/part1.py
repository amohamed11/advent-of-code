with open("input.txt") as fin:
    longstring = fin.read().strip()
    numbers = longstring.split('\n')
    numbers.sort()
    for i in range(len(numbers)-1):
        for j in range(i+1, len(numbers)):
            num1 = int(numbers[i])
            num2 = int(numbers[j])
            summed = num1 + num2

            if summed == 2020:
                multiplied = num1 * num2
                print("Part 1:", multiplied)
                quit()

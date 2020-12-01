with open("input.txt") as fin:
    longstring = fin.read().strip()
    numbers = longstring.split('\n')
    numbers.sort()
    for i in range(len(numbers)-2):
        for j in range(i+1, len(numbers)-1):
            num1 = int(numbers[i])
            num2 = int(numbers[j])
            summed = num1 + num2

            if (summed < 2020):
                for k in range(j+1, len(numbers)):
                    num3 = int(numbers[k])
                    summed = num1 + num2 + num3

                    if summed == 2020:
                        multiplied = num1 * num2 * num3
                        print("Part 2:", multiplied)
                        quit()

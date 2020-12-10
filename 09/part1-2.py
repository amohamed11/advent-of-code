PREAMBLE_LENGTH = 25

def findEncryptionWeakness(num, numbers):
    numbers.remove(num)

    for setSize in range(2, len(numbers)):
        for i in range(setSize-1, len(numbers)):
            if num > numbers[i]:
                summation = sum(numbers[i-setSize:i])
                if summation == num:
                    smallest = min(numbers[i-setSize:i])
                    largest = max(numbers[i-setSize:i])
                    print(f"Encryption weakness: {smallest + largest}")
                    quit()


def checkSummable(num, preamble):
    for i in range(len(preamble)-1, -1, -1):
        if num > preamble[i]:
            for k in range(i):
                summation = preamble[i] + preamble[k]
                # print(i,k, summation)
                if summation == num:
                    return True

    return False


def main():
    with open("input.txt", "r") as fin:
        lines = fin.read().splitlines()
        numbers = list(int(x) for x in lines)
        unsummable = 0

        for i in range(PREAMBLE_LENGTH, len(numbers)):
            num = numbers[i]
            preamble = numbers[i-PREAMBLE_LENGTH:i]
            preamble.sort()

            if checkSummable(num, preamble) == False:
                print("Found unsummable number:", num)
                unsummable = num
                break

        findEncryptionWeakness(unsummable, numbers)

main()
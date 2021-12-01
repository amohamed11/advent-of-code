ACCUMULATOR = 0

def handleOperation(instruction, currentIndex):
    global ACCUMULATOR

    operation = instruction.split(' ')[0]
    number = int(instruction.split(' ')[1])

    if operation == "nop":
        pass
    elif operation == "acc":
        ACCUMULATOR += number
    elif operation == "jmp":
        jumpTo = currentIndex + number
        return jumpTo-1

    return currentIndex


def main():
    global ACCUMULATOR

    with open("input.txt", "r") as fin:
        instructions = fin.read().splitlines()
        index = 0
        instructionsDone = []
        
        while index < len(instructions):
            if index in instructionsDone:
                print("Loop found ... aborting")
                print("Accumulator value:", ACCUMULATOR)
                quit()

            instruction = instructions[index]
            instructionsDone.append(index)

            operation = instruction.split(' ')[0]
            argument = int(instruction.split(' ')[1])

            index = handleOperation(instruction, index)
            index += 1
        
    print("Accumulator value:", ACCUMULATOR)

main()
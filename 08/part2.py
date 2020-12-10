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
        modifiedInstructions = instructions.copy()
        stillLooping = True
        gatherModifiable = True
        modifiable = []
        
        while stillLooping:
            stillLooping = False
            index = 0
            instructionsDone = []
            ACCUMULATOR = 0

            while index < len(modifiedInstructions):
                if index in instructionsDone:
                    stillLooping = True
                    break

                instruction = modifiedInstructions[index]
                instructionsDone.append(index)

                operation = instruction.split(' ')[0]
                argument = int(instruction.split(' ')[1])

                if gatherModifiable and (operation == "jmp" or operation == "nop"):
                    modifiable.append(index)

                index = handleOperation(instruction, index)
                index += 1

            if stillLooping == False:
                break

            gatherModifiable = False
            indexToModifiy = modifiable.pop()
            instructionToModifiy = instructions[indexToModifiy]
            modifiedInstructions = instructions.copy()

            if instructionToModifiy[:3] == "jmp":
                modifiedInstructions[indexToModifiy] = "nop"+instructionToModifiy[3:]
            elif instructionToModifiy[:3] == "nop":
                modifiedInstructions[indexToModifiy] = "jmp"+instructionToModifiy[3:]
        
    print("Accumulator value:", ACCUMULATOR)

main()
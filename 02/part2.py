with open("input.txt") as fin:
    lines = fin.read().splitlines()
    validPasswords = 0
    for line in lines:
        linesplit = line.split(':')
        policy = linesplit[0].split(' ')
        password = linesplit[1].lstrip()
        
        positions = policy[0].split('-')
        pos1 = int(positions[0])-1
        pos2 = int(positions[1])-1

        character = policy[1]

        correctCount = 0
        if password[pos1] == character:
            correctCount += 1
        if password[pos2] == character:
            correctCount += 1
        
        if correctCount == 1:
            validPasswords += 1
    
    print("Valid passwords:", validPasswords)

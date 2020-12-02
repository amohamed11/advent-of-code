with open("input.txt") as fin:
    lines = fin.read().splitlines()
    validPasswords = 0
    for line in lines:
        linesplit = line.split(':')
        policy = linesplit[0].split(' ')
        password = linesplit[1].lstrip()
        
        limits = policy[0].split('-')
        minimum = int(limits[0])
        maximum = int(limits[1])

        character = policy[1]

        if (password.count(character) in range(minimum, maximum+1)):
            validPasswords += 1
    
    print("Valid passwords:", validPasswords)

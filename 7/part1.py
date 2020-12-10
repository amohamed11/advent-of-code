import re

with open("input.txt", "r") as fin:
    rules = fin.read()
    bagsVisited = []
    validBags = ["shiny gold"]
    validBagsCount = 0

    while len(validBags) != 0:
        color = validBags.pop()
        pattern = r".*.%s.*" % (color)
        matches = re.findall(pattern, rules)
        for match in matches:
            bagColor = ' '.join(match.split(' ')[:2])
            if bagColor not in bagsVisited:
                validBags.append(bagColor)
                bagsVisited.append(bagColor)
                validBagsCount += 1

    print("Possible bags count:", validBagsCount)
import re

# INCORRECT

with open("input.txt", "r") as fin:
    rules = fin.read()
    bagsVisited = {"shiny gold": 0}
    validBags = ["shiny gold"]
    totalBagsCount = 1

    while len(validBags) != 0:
        color = validBags.pop()
        bagCount = 0
        pattern = r"^%s.*" % (color)
        matches = re.findall(pattern, rules, re.MULTILINE)
        # print(color)
        for match in matches:
            subMatches = re.findall(r"\d.\w+.\w+(?=\sbag[s]?\b)", match)
            for i in range(len(subMatches)):
                subBagCountColors = ' '.join(subMatches[i].split(' '))
                subBagColor = subBagCountColors[1:].lstrip()
                # print(subBagCountColors)
                subBagCount = int(subBagCountColors[0].rstrip())

                if subBagColor not in bagsVisited:
                    validBags.append(subBagColor)
                bagsVisited[subBagColor] = subBagCount
                bagCount += subBagCount

                # print(subBagCountColors)
        if bagCount != 0:
            print("%d + %d*%d" % (bagsVisited[color], bagsVisited[color], bagCount))
            totalBagsCount += bagsVisited[color] * bagCount
    
    print("Total bags count:", totalBagsCount)
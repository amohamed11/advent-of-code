with open("input.txt", "r") as fin:
    joltDifferences = {1: [], 2: [], 3: []}
    prev = 0

    lines = fin.read().splitlines()
    adapters = list(int(x) for x in lines)
    adapters.sort()

    for adapter in adapters:
        diff = adapter-prev
        if diff <= 3 and diff > 0:
            joltDifferences[diff].append(prev)
        prev = adapter

    joltDifferences[3].append(adapters[-1]+3)

    print(f"Diff of 1: {len(joltDifferences[1])}")
    print(f"Diff of 3: {len(joltDifferences[3])}")
    print(f"Result: {len(joltDifferences[1]) * len(joltDifferences[3])}")
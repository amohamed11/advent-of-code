with open("input.txt", "r") as fin:
    allAnswersCount = 0
    consensusAnswersCount = 0
    groups = fin.read().split('\n\n')
    for group in groups:
        allAnswers = []
        answerCounts = {}
        groupAnswers = group.replace('\n', '')
        for answer in groupAnswers:
            if answer not in allAnswers:
                allAnswers.append(answer)
            if answer in answerCounts:
                answerCounts[answer] += 1
            else:
                answerCounts[answer] = 1
        
        consensusCount = len([k for k,v in answerCounts.items() if v == group.count('\n')+1])
        print(consensusCount)
        consensusAnswersCount += consensusCount
        allAnswersCount += len(allAnswers)
    
    print("All yes answers:", allAnswersCount)
    print("Consensus yes answers:", consensusAnswersCount)
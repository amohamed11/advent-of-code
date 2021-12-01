NUM_ROWS = range(128)
NUM_COLS = range(8)

with open("input.txt", "r") as fin:
    highestSeatID = 0
    seatIDs = []
    tickets = fin.read().splitlines()
    for ticket in tickets:
        row = list(NUM_ROWS)
        col = list(NUM_COLS)
        for letter in ticket.lower():
            if letter == 'f':
                row = row[:len(row)//2]
            elif letter == 'b':
                row = row[len(row)//2:]
            elif letter == 'r':
                col = col[len(col)//2:]
            elif letter == 'l':
                col = col[:len(col)//2]
        
        seatID = row[0] * 8 + col[0]
        seatIDs.append(seatID)
        if seatID > highestSeatID:
            highestSeatID = seatID
    
    missingID = set(list(range(1, highestSeatID+1))) - set(seatIDs)
    print("Missing seat ID:", missingID)
    print("Highest seat ID:", highestSeatID)
with open("input.txt") as fin:
    valid = 0
    required = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
    optional = set(["cid"])

    passports = fin.read().split('\n\n')
    for passport in passports:
        passport = passport.replace('\n', ' ')
        fieldsFound = []
        fields = passport.split(' ')
        if len(fields) < len(required):
            pass
        for field in fields:
            fieldPair = field.split(':')
            fieldsFound.append(fieldPair[0])

        keysFound = set(fieldsFound) 
        if required == keysFound or required.union(optional) == keysFound:
            valid += 1
    
    print("Valid passports:", valid)
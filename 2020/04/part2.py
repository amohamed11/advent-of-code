import re

# this was regex hell, and I did not enjoy it
with open("input.txt") as fin:
    valid = 0
    required = set(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
    requiredRules = {
        "byr": "^19[2-9][0-9]$|^200[0-2]$",
        "iyr": "^20[1-2][0-9]$",
        "eyr": "^20([2][0-9]|30)$",
        "hgt": "^(1([5|6|7|8][0-9]|9[0-3])cm|([5|6][0-9]|7[0-6])in)$",
        "hcl": "^\#([0-9]|[a-f]){6}$",
        "ecl": "^amb|blu|brn|gry|grn|hzl|oth$",
        "pid": "^[0-9]{9}$"
    }
    optional = set(["cid"])

    passports = fin.read().split('\n\n')
    for passport in passports:
        validFields = 0
        fieldsFound = []
        passport = passport.replace('\n', ' ')
        fields = passport.split(' ')

        if len(fields) < len(required):
            pass
        for field in fields:
            fieldPair = field.split(':')
            if fieldPair[0] != "cid" and re.match(requiredRules[fieldPair[0]], fieldPair[1]) != None:
                validFields += 1
            fieldsFound.append(fieldPair[0])

        keysFound = set(fieldsFound) 
        if required == keysFound or required.union(optional) == keysFound:
            if validFields == len(requiredRules):
                valid += 1
    
    print("Valid passports:", valid)
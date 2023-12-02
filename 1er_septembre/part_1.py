def first_and_last_number(lineToLookThrough: str) -> int:
    """search for the first and the last number in a str and combine them to create a number"""
    firstDigit: str = None
    secondDigit: str = None
    
    for letter in lineToLookThrough:
        if letter.isdigit():
            if firstDigit is None:
                firstDigit = letter
                continue
            secondDigit = letter

    if secondDigit != None:
        result: int = int(firstDigit + secondDigit)
    else:
        result: int = int(firstDigit*2)
    
    if result > 99:
        print(firstDigit)
        print(secondDigit)
        print(result)

    return result

if __name__ == "__main__":
    result: int = 0
    with open("puzzle.txt", "r") as puzzle:
        line: str = puzzle.readline()
    
        while line:
            result += first_and_last_number(line)
            line = puzzle.readline()
    print(result)
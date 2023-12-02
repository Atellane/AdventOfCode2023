from part_1 import first_and_last_number

def convert_special_digit(word: str) -> str:
    """convert special digits written in words to numerical digits"""
    # the first 8 weird number are to handle the case when two number touch each other (example : "twone" -> "two" + "one" -> "21")
    special_digits = {
        'twone': '21',
        'eightwo': '82',
        'eighthree': '83',
        'oneight': '18',
        'threeight': '38',
        'fiveight': '58',
        'nineight': '98',
        'sevenine': '79',
        'one': '1',
        'two': '2',
        'three': '3',
        'four': '4',
        'five': '5',
        'six': '6',
        'seven': '7',
        'eight': '8',
        'nine': '9',
    }

    for word_digit, numerical_digit in special_digits.items():
        # replace() replace the part of the str, mentioned by its first parameter, by its second parameter without compromising other parts of the str
        word = word.replace(word_digit, numerical_digit)

    return word

assert convert_special_digit("sevenine") == "79", "miaou"
result: int = 0
with open("puzzle.txt", "r") as puzzle:
    line: str = puzzle.readline()
    
    while line:
        result += first_and_last_number(convert_special_digit(line))
        line = puzzle.readline()
print(result)
values = {"M": 1000, "D": 500, "C": 100, "L": 50, "X": 10, "V": 5, "I": 1}


def parse_roman_numeral(numeral: str) -> int:
    value = 0
    last = 0
    for n in reversed(numeral):
        n = values[n]
        if last > n:
            value -= n
        else:
            value += n
        last = n
    return value


numerals = {
    1: "I",
    2: "II",
    3: "III",
    4: "IV",
    5: "V",
    6: "VI",
    7: "VII",
    8: "VIII",
    9: "IX",
    10: "X",
    20: "XX",
    30: "XXX",
    40: "XL",
    50: "L",
    60: "LX",
    70: "LXX",
    80: "LXXX",
    90: "XC",
    100: "C",
    200: "CC",
    300: "CCC",
    400: "CD",
    500: "D",
    600: "DC",
    700: "DCC",
    800: "DCCC",
    900: "CM",
    1000: "M",
}


def write_as_roman(number: int) -> str:
    bits = []
    while number:
        next_value, next_numeral = max(
            (value, numeral) for value, numeral in numerals.items() if value <= number
        )
        number -= next_value
        bits.append(next_numeral)
    return "".join(bits)


def solution() -> int:
    characters_saved = 0
    with open("data/0089_roman.txt") as f:
        for line in f:
            long = line.strip()
            short = write_as_roman(parse_roman_numeral(long))
            characters_saved += len(long) - len(short)
    return characters_saved


if __name__ == "__main__":
    import python.src.project_euler_python.runner as runner

    runner.run(globals())

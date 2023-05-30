def number_to_word(number: int) -> str:
    special_cases = {
        1: "one",
        2: "two",
        3: "three",
        4: "four",
        5: "five",
        6: "six",
        7: "seven",
        8: "eight",
        9: "nine",
        10: "ten",
        11: "eleven",
        12: "twelve",
        13: "thirteen",
        14: "fourteen",
        15: "fifteen",
        16: "sixteen",
        17: "seventeen",
        18: "eighteen",
        19: "nineteen",
    }
    if number in special_cases:
        return special_cases[number]

    if number < 100:
        tens = {
            2: "twenty",
            3: "thirty",
            4: "forty",
            5: "fifty",
            6: "sixty",
            7: "seventy",
            8: "eighty",
            9: "ninety",
        }
        return tens[number // 10] + (
            "" if number % 10 == 0 else number_to_word(number % 10)
        )

    if number < 1000:
        return (
            number_to_word(number // 100)
            + "hundred"
            + ("" if number % 100 == 0 else "and" + number_to_word(number % 100))
        )

    if number < 10000:
        return (
            number_to_word(number // 1000)
            + "thousand"
            + ("" if number % 1000 == 0 else "and" + number_to_word(number % 1000))
        )


def solution() -> int:
    words = [number_to_word(number) for number in range(1, 1001)]
    return sum(map(len, words))


if __name__ == "__main__":
    import runner

    runner.run(globals())

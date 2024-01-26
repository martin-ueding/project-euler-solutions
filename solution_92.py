from tqdm import tqdm


terminators = {}


def get_terminator(number: int) -> int:
    passed = []
    while True:
        if number == 1 or number == 89:
            break
        if number in terminators:
            number = terminators[number]
            break
        passed.append(number)
        number = sum(int(digit) ** 2 for digit in str(number))
    for p in passed:
        terminators[p] = number
    return number


def solution() -> int:
    terminates_in_89 = 0
    for number in tqdm(range(1, 10_000_000)):
        terminator = get_terminator(number)
        if terminator == 89:
            terminates_in_89 += 1
    return terminates_in_89


if __name__ == "__main__":
    import runner

    runner.run(globals())

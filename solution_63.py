import itertools


def solution() -> int:
    result = 0
    for base in range(1, 10):
        for exponent in itertools.count(1):
            num_digits = len(str(base**exponent))
            if num_digits == exponent:
                result += 1
            elif num_digits < exponent:
                break
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

import itertools


def has_same_digits(first: int, second: int) -> bool:
    return sorted(str(first)) == sorted(str(second))


def solution() -> int:
    for digits in itertools.count(1):
        floor = 10 ** (digits - 1)
        ceiling = 10**digits // 6
        for number in range(floor, ceiling):
            for factor in range(2, 7):
                if not has_same_digits(number, number * factor):
                    break
            else:
                return number


if __name__ == "__main__":
    import runner

    runner.run(globals())

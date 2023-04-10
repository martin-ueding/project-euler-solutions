import itertools
from problem_7 import prime_sieve


def is_pandigital(number: int) -> bool:
    s = str(number)
    if "0" in s:
        return False
    digits = set(s)
    return len(digits) == len(s)


def solution() -> int:
    return max(
        itertools.islice(filter(is_pandigital, reversed(prime_sieve(7_654_321))), 1)
    )


if __name__ == "__main__":
    import runner

    runner.run(globals())

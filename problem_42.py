from problem_7 import prime_sieve


def is_pandigital(number: int) -> bool:
    s = str(number)
    if "0" in s:
        return False
    digits = set(s)
    return len(digits) == len(s)


def solution() -> int:
    return max(filter(is_pandigital, prime_sieve(7_654_321)))


if __name__ == "__main__":
    import runner

    runner.run(globals())

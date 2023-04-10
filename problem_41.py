import itertools

from problem_7 import prime_sieve


def is_pandigital(number: int) -> bool:
    s = str(number)
    if "0" in s:
        return False
    digits = set(s)
    return len(digits) == len(s)


def _solution_sieve() -> int:
    return max(
        itertools.islice(filter(is_pandigital, reversed(prime_sieve(7_654_321))), 1)
    )


def is_prime(number: int) -> bool:
    for divisor in itertools.count(2):
        if number % divisor == 0:
            return False
        if divisor * divisor > number:
            return True


def solution_permutations() -> int:
    for digits in reversed(list(itertools.permutations("1234567"))):
        number = int("".join(digits))
        if is_prime(number):
            return number


if __name__ == "__main__":
    import runner

    runner.run(globals())

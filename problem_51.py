import itertools
from problem_7 import prime_sieve


def family_size(prime: int, prime_set: set[int]) -> int:
    digits = list(str(prime))
    result = 0
    for positions in itertools.product(*[(i, None) for i in range(len(digits) - 1)]):
        copy = list(digits)
        for replacement in range(10):
            for position in positions:
                if position is None:
                    continue
                if position == 0 and replacement == 0:
                    continue
                copy[position] = str(replacement)
                number = int("".join(copy))
                if number in prime_set:
                    result += 1
    return result


def solution() -> int:
    primes = prime_sieve(100000)
    prime_set = set(primes)
    for prime in primes:
        if family_size(prime, prime_set) == 6:
            return prime


if __name__ == "__main__":
    import runner

    runner.run(globals())

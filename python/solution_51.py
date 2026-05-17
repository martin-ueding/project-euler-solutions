import itertools
from solution_7 import prime_sieve


def get_prime_family(digits: list[str], mask: tuple, prime_set: set[int]) -> list[int]:
    new_numbers = [
        int("".join(str(replacement) if m else digit for digit, m in zip(digits, mask)))
        for replacement in range(1 if mask[0] else 0, 10)
    ]
    return [number for number in new_numbers if number in prime_set]


def get_prime_families(prime: int, prime_set: set[int]) -> list[list[int]]:
    digits = list(str(prime))
    families = [
        get_prime_family(digits, mask + (False,), prime_set)
        for mask in itertools.product(*[(True, False) for i in range(len(digits) - 1)])
    ]
    result = [family for family in families if family]
    result.sort()
    return result



def solution() -> int:
    primes = prime_sieve(1000000)
    prime_set = set(primes)
    for prime in primes:
        families = get_prime_families(prime, prime_set)
        for family in families:
            if len(family) == 8:
                return family[0]


if __name__ == "__main__":
    import runner

    runner.run(globals())

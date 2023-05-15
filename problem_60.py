import collections
from problem_7 import prime_sieve

ceiling = 10_000


def has_property(first: int, second: int, prime_set: set[int]) -> bool:
    number_1 = int(str(first) + str(second))
    number_2 = int(str(second) + str(first))
    assert number_1 <= ceiling
    assert number_2 <= ceiling
    return number_1 in prime_set and number_2 in prime_set


def find_tuple_of(size: int) -> int:
    partners = collections.defaultdict(list)
    primes = prime_sieve(ceiling)
    prime_set = set(primes)
    for first_prime in primes:
        for second_prime in primes:
            if second_prime == first_prime:
                break
            if has_property(first_prime, second_prime, prime_set):
                partners[second_prime].append(first_prime)
    print(partners)


def solution() -> None:
    return find_tuple_of(5)


if __name__ == "__main__":
    import runner

    runner.run(globals())

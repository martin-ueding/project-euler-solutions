from solution_7 import prime_sieve
from solution_62 import sort_digits


def are_permutations(left: int, right: int) -> bool:
    return sort_digits(left) == sort_digits(right)


def solution() -> int:
    primes = prime_sieve(10**5)
    values = []
    for p1 in primes:
        for p2 in primes:
            if p1 == p2:
                break
            number = p1 * p2
            if number > 10**7:
                break
            t = (p1 - 1) * (p2 - 1)
            if are_permutations(t, number):
                values.append((number / t, number))
    return min(values)[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())

import math
from problem_7 import prime_sieve


def solution() -> int:
    ceiling = 10_000
    conjecture_holds = [False] * ceiling
    conjecture_holds[0] = True
    conjecture_holds[1] = True
    primes = prime_sieve(ceiling)
    for prime in primes:
        conjecture_holds[prime] = True
        for i in range(1, int(math.sqrt((ceiling - prime) / 2)) + 1):
            number = prime + 2 * i**2
            if number < ceiling:
                conjecture_holds[number] = True
    for number, holds in enumerate(conjecture_holds):
        if not holds and number % 2 == 1:
            return number


if __name__ == "__main__":
    import runner

    runner.run(globals())

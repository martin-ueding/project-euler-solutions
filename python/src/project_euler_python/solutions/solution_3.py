from ..primes import PrimeList


number = 600851475143


def solution_reducing() -> int:
    remainder = number
    last_factor = 0
    for prime in PrimeList():
        while remainder % prime == 0:
            last_factor = prime
            remainder /= prime
        if remainder == 1:
            break
    return last_factor

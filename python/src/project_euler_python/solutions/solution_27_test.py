from ..primes import PrimeSet


def test_prime_set() -> None:
    prime_set = PrimeSet()
    assert prime_set.contains(7)
    assert not prime_set.contains(9)

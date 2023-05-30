from solution_51 import get_prime_family
from solution_7 import prime_sieve


def test_get_prime_family() -> None:
    primes = prime_sieve(100000)
    prime_set = set(primes)
    assert get_prime_family(
        list("56003"), (False, False, True, True, False), prime_set
    ) == [56003, 56113, 56333, 56443, 56663, 56773, 56993]

from problem_60 import find_tuple_of, has_property, ceiling
from problem_7 import prime_sieve


def test_has_property() -> None:
    primes = prime_sieve(ceiling)
    prime_set = set(primes)
    assert has_property(3, 7, prime_set)
    assert has_property(109, 3, prime_set)
    assert has_property(3, 673, prime_set)
    assert not has_property(3, 12, prime_set)


def test_solution() -> None:
    assert find_tuple_of(4) == 792

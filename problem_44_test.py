import itertools
from problem_44 import is_pentagonal, iter_pentagonal_numbers


def test_is_pentagonal() -> None:
    assert is_pentagonal(1)
    assert is_pentagonal(5)
    assert is_pentagonal(12)
    assert is_pentagonal(22)
    assert not is_pentagonal(2)
    assert not is_pentagonal(3)
    assert not is_pentagonal(4)

    for p in itertools.islice(iter_pentagonal_numbers(), 1_000_000):
        assert is_pentagonal(p)

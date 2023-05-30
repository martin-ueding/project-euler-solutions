import itertools

from solution_58 import iter_diagonals


def test_iter_diagonals() -> None:
    expected = [[3, 5, 7, 9], [13, 17, 21, 25], [31, 37, 43, 49]]
    actual = list(itertools.islice(iter_diagonals(), len(expected)))
    assert actual == expected

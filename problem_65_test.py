import itertools
from problem_65 import continued_fraction_e, convergent_from_sequence


def test_convergent_from_sequence() -> None:
    assert convergent_from_sequence([1, 2]) == (3, 2)
    assert convergent_from_sequence([1, 2, 2]) == (7, 5)
    assert convergent_from_sequence([1, 2, 2, 2]) == (17, 12)
    assert convergent_from_sequence([1, 2, 2, 2, 2]) == (41, 29)


def test_continued_fraction_e() -> None:
    expected = [2, 1, 2, 1, 1, 4, 1, 1, 6, 1]
    actual = list(itertools.islice(continued_fraction_e(), len(expected)))
    assert actual == expected
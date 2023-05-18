import itertools
from problem_65 import (
    continued_fraction_e,
    continued_fraction_sqrt_2,
    convergent_from_sequence,
    convergents_series,
)


def test_convergent_from_sequence() -> None:
    assert convergent_from_sequence([1, 2]) == (3, 2)
    assert convergent_from_sequence([1, 2, 2]) == (7, 5)
    assert convergent_from_sequence([1, 2, 2, 2]) == (17, 12)
    assert convergent_from_sequence([1, 2, 2, 2, 2]) == (41, 29)


def test_continued_fraction_e() -> None:
    expected = [2, 1, 2, 1, 1, 4, 1, 1, 6, 1]
    actual = list(itertools.islice(continued_fraction_e(), len(expected)))
    assert actual == expected


def test_convergents_series_sqrt_2() -> None:
    expected = [
        (1, 1),
        (3, 2),
        (7, 5),
        (17, 12),
        (41, 29),
        (99, 70),
        (239, 169),
        (577, 408),
        (1393, 985),
        (3363, 2378),
    ]
    actual = list(
        itertools.islice(convergents_series(continued_fraction_sqrt_2()), len(expected))
    )
    assert actual == expected


def test_convergents_series_e() -> None:
    expected = [
        (2, 1),
        (3, 1),
        (8, 3),
        (11, 4),
        (19, 7),
        (87, 32),
        (106, 39),
        (193, 71),
        (1264, 465),
        (1457, 536),
    ]
    actual = list(
        itertools.islice(convergents_series(continued_fraction_e()), len(expected))
    )
    assert actual == expected

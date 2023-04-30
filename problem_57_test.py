import itertools

from problem_57 import square_root_series


def test_square_root_series() -> None:
    elems = list(itertools.islice(square_root_series(), 4))
    assert elems == [(3, 2), (7, 5), (17, 12), (41, 29)]

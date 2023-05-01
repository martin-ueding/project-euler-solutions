from problem_81 import iter_diagonally


def test_iter_diagonally() -> None:
    expected = [(2, 2), (2, 1), (1, 2), (2, 0), (1, 1), (0, 2), (1, 0), (0, 1), (0, 0)]
    actual = list(iter_diagonally(3))
    assert actual == expected

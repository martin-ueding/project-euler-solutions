from problem_71 import bisect_numerator, find_next_smaller_fraction


def test_find_next_smaller_fraction() -> None:
    assert find_next_smaller_fraction((3, 7), 8) == (2, 5)


def test_bisect_numerator() -> None:
    assert bisect_numerator((3, 7), 5) == 2

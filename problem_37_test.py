from problem_37 import iter_truncations


def test_iter_truncations() -> None:
    truncations = list(iter_truncations(1234))
    assert truncations == [234, 123, 34, 12, 4, 1]

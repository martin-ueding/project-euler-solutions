from solution_86 import integer_root, path_length, shortest_path_is_integer


def test_path_length() -> None:
    assert path_length(6, 5, 3) == 10


def test_shortest_path_is_integer() -> None:
    assert shortest_path_is_integer(6, 5, 3)


def test_integer_root() -> None:
    assert integer_root(255) is None
    assert integer_root(256) == 16
    assert integer_root(257) is None

    assert integer_root(999_999_999_999_999_999_999_999) is None
    assert integer_root(1_000_000_000_000_000_000_000_000) == 1_000_000_000_000
    assert integer_root(1_000_000_000_000_000_000_000_001) is None

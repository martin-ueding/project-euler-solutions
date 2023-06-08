from solution_86 import integer_root, multiplicity, shortest_path_is_integer


def test_shortest_path_is_integer() -> None:
    assert shortest_path_is_integer(6, 5, 3)


def test_integer_root() -> None:
    assert integer_root(255) is None
    assert integer_root(256) == 16
    assert integer_root(257) is None

    assert integer_root(999_999_999_999_999_999_999_999) is None
    assert integer_root(1_000_000_000_000_000_000_000_000) == 1_000_000_000_000
    assert integer_root(1_000_000_000_000_000_000_000_001) is None


def test_multiplicity() -> None:
    expected = {2: 1, 3: 1, 4: 2, 5: 2, 6: 3, 7: 3, 8: 3, 9: 2, 10: 2, 11: 1, 12: 1}
    for b_plus_c, expected_multiplicity in expected.items():
        assert multiplicity(6, b_plus_c) == expected_multiplicity

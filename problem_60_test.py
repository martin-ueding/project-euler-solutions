from problem_60 import find_tuple_of, has_property


def test_has_property() -> None:
    assert has_property(3, 7)
    assert has_property(109, 3)
    assert not has_property(3, 12)


def test_solution() -> None:
    assert find_tuple_of(4) == 792

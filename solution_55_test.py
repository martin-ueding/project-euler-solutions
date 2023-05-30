from solution_55 import is_lychrel, reverse_and_add


def test_reverse_and_add() -> None:
    assert reverse_and_add(47) == 121


def test_is_lychrel() -> None:
    assert is_lychrel(47)
    assert is_lychrel(349)
    assert not is_lychrel(196)

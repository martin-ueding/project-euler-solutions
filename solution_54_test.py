from solution_54 import is_straight, parse_hand, player_1_wins


def test_parse_hand() -> None:
    hand = "5H 5C 6S 7S KD"
    expected = [(5, "H"), (5, "C"), (6, "S"), (7, "S"), (13, "D")]
    assert parse_hand(hand) == expected


def test_is_straight() -> None:
    hand = [(2, "H"), (3, "H"), (4, "H"), (5, "C"), (6, "H")]
    expected = [6, 5, 4, 3, 2]
    assert is_straight(hand) == expected


def test_with_examples() -> None:
    assert not player_1_wins("5H 5C 6S 7S KD 2C 3S 8S 8D TD")
    assert player_1_wins("5D 8C 9S JS AC 2C 5C 7D 8S QH")
    assert not player_1_wins("2D 9C AS AH AC 3D 6D 7D TD QD")
    assert player_1_wins("4D 6S 9H QH QC 3D 6D 7H QD QS")
    assert player_1_wins("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D")

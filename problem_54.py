import collections


def map_value(value: str) -> int:
    try:
        return int(value)
    except ValueError:
        return {"T": 10, "J": 11, "Q": 12, "K": 13, "A": 14}[value]


def parse_hand(hand: str) -> list[tuple[int, str]]:
    cards = hand.split()
    return [(map_value(pair[0]), pair[1]) for pair in cards]


def test_parse_hand() -> None:
    hand = "5H 5C 6S 7S KD"
    expected = [(5, "H"), (5, "C"), (6, "S"), (7, "S"), (13, "D")]
    assert parse_hand(hand) == expected


def is_high_card(hand: list[tuple[int, str]]) -> bool:
    return True


def is_one_pair(hand: list[tuple[int, str]]) -> bool:
    counts = collections.Counter(hand)
    return 2 in counts.values()


def is_two_pairs(hand: list[tuple[int, str]]) -> bool:
    counts = collections.Counter(hand)
    counts_2 = collections.Counter(counts.values())
    return counts_2[2] == 2


def is_three_of_a_kind(hand: list[tuple[int, str]]) -> bool:
    value_counts = collections.Counter(value for value, suit in hand)
    return 3 in value_counts.items()


def is_straight(hand: list[tuple[int, str]]) -> bool:
    pass


def is_flush(hand: list[tuple[int, str]]) -> bool:
    pass


def is_full_house(hand: list[tuple[int, str]]) -> bool:
    pass


def is_four_of_a_kind(hand: list[tuple[int, str]]) -> bool:
    value_counts = collections.Counter(value for value, suit in hand)
    return 3 in value_counts.items()


def is_straight_flush(hand: list[tuple[int, str]]) -> bool:
    pass


def is_royal_flush(hand: list[tuple[int, str]]) -> bool:
    pass


def rate_special_hand(hand: list[tuple[int, str]]) -> int:
    predicates = [
        is_high_card,
        is_one_pair,
        is_two_pairs,
        is_three_of_a_kind,
        is_straight,
        is_flush,
        is_full_house,
        is_four_of_a_kind,
        is_straight_flush,
        is_royal_flush,
    ]
    for index, predicate in reversed(list(enumerate(predicates))):
        if predicate(hand):
            return index


def make_rating(hand: list[tuple[int, str]]) -> list[int]:
    return [rate_special_hand(hand)] + [value for value, suit in hand]


def player_1_wins(line: str) -> bool:
    cards = parse_hand(line.strip())
    hand_1 = cards[:5]
    hand_2 = cards[5:]
    return make_rating(hand_1) > make_rating(hand_2)


def solution() -> int:
    wins_player_1 = 0
    with open("data/p054_poker.txt") as f:
        for line in f:
            if player_1_wins(line):
                wins_player_1 += 1
    return wins_player_1



if __name__ == "__main__":
    import runner

    runner.run(globals())

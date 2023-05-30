import itertools
from typing import Optional


def map_value(value: str) -> int:
    try:
        return int(value)
    except ValueError:
        return {"T": 10, "J": 11, "Q": 12, "K": 13, "A": 14}[value]


def parse_hand(hand: str) -> list[tuple[int, str]]:
    cards = hand.split()
    return [(map_value(pair[0]), pair[1]) for pair in cards]


def group_card_values(hand: list[tuple[int, str]]) -> list[list[int]]:
    values = [value for value, suit in hand]
    values.sort(reverse=True)
    groups = [list(group) for key, group in itertools.groupby(values)]
    groups.sort(key=lambda group: (len(group), group[0]), reverse=True)
    return groups


def is_high_card(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    return sorted((value for value, suit in hand), reverse=True)


def flatten(xss: list[list[int]]) -> list[int]:
    return [x for xs in xss for x in xs]


def is_one_pair(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    groups = group_card_values(hand)
    if len(groups[0]) == 2:
        return flatten(groups)


def is_two_pairs(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    groups = group_card_values(hand)
    if len(groups[0]) == 2 and len(groups[1]) == 2:
        return flatten(groups)


def is_three_of_a_kind(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    groups = group_card_values(hand)
    if len(groups[0]) == 3:
        return flatten(groups)


def is_straight(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    values = [value for value, suit in hand]
    values.sort(reverse=True)
    expected = list(reversed(range(min(values), max(values) + 1)))
    if values == expected:
        return values


def is_flush(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    if len({suit for value, suit in hand}) == 1:
        return sorted((value for value, suit in hand), reverse=True)


def is_full_house(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    groups = group_card_values(hand)
    if len(groups[0]) == 3 and len(groups[1]) == 2:
        return flatten(groups)


def is_four_of_a_kind(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    groups = group_card_values(hand)
    if len(groups[0]) == 4:
        return flatten(groups)


def is_straight_flush(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    if is_straight(hand):
        return is_flush(hand)


def is_royal_flush(hand: list[tuple[int, str]]) -> Optional[list[int]]:
    if max(value for value, suit in hand) == 14:
        return is_straight_flush(hand)


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
        if result := predicate(hand):
            return [index, predicate.__name__] + result


def player_1_wins(line: str) -> bool:
    cards = parse_hand(line.strip())
    hand_1 = cards[:5]
    hand_2 = cards[5:]
    hand_1.sort()
    hand_2.sort()
    rating_1 = rate_special_hand(hand_1)
    rating_2 = rate_special_hand(hand_2)
    return rating_1 > rating_2


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

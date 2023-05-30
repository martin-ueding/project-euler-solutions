import collections
import itertools
import random
from typing import Iterator

board = [
    "GO",
    "A1",
    "CC1",
    "A2",
    "T1",
    "R1",
    "B1",
    "CH1",
    "B2",
    "B3",
    "JAIL",
    "C1",
    "U1",
    "C2",
    "C3",
    "R2",
    "D1",
    "CC2",
    "D2",
    "D3",
    "FP",
    "E1",
    "CH2",
    "E2",
    "E3",
    "R3",
    "F1",
    "F2",
    "U2",
    "F3",
    "G2J",
    "G1",
    "G2",
    "CC3",
    "G3",
    "R4",
    "CH3",
    "H1",
    "T2",
    "H2",
]


def dice_pair(eyes: int) -> Iterator[tuple[int, bool]]:
    first = random.randint(1, eyes)
    second = random.randint(1, eyes)
    return first + second, first == second


class Movement:
    def seek(self, position: int) -> int:
        raise NotImplementedError()


class ForwardMovement(Movement):
    def __init__(self, prefix: str) -> None:
        self._prefix = prefix

    def seek(self, position: int) -> int:
        while not board[position].startswith(self._prefix):
            position += 1
            position %= len(board)
        return position


class BackwardMovement(Movement):
    def seek(self, position: int) -> int:
        return (position - 3 + len(board)) % len(board)


class NullMovement(Movement):
    def seek(self, position: int) -> int:
        return position


def card_stack(cards: list[Movement]) -> Iterator[Movement]:
    random.shuffle(cards)
    for card in itertools.cycle(cards):
        yield card


def chance_cards() -> Iterator[Movement]:
    cards = [
        ForwardMovement("GO"),
        ForwardMovement("JAIL"),
        ForwardMovement("C1"),
        ForwardMovement("E3"),
        ForwardMovement("H2"),
        ForwardMovement("R1"),
        ForwardMovement("R"),
        ForwardMovement("R"),
        ForwardMovement("U"),
        BackwardMovement(),
    ] + [NullMovement()] * 6
    yield from card_stack(cards)


def community_chest_cards() -> Iterator[Movement]:
    cards = [ForwardMovement("GO"), ForwardMovement("JAIL")] + [NullMovement()] * 14
    yield from card_stack(cards)


def solution() -> int:
    position = 0
    visited_fields = collections.defaultdict(lambda: 0)
    chance_cards_iter = chance_cards()
    community_chest_cards_iter = community_chest_cards()
    steps = 4000000
    for i in range(steps):
        eyes, is_double = dice_pair(4)
        position += eyes
        position %= len(board)

        if board[position].startswith("CC"):
            movement = next(community_chest_cards_iter)
            position = movement.seek(position)
        elif board[position].startswith("CH"):
            movement = next(chance_cards_iter)
            position = movement.seek(position)
        elif board[position] == "G2J":
            position = board.index("JAIL")

        visited_fields[board[position]] += 1

    results = sorted(
        [
            (100 * count / steps, field, board.index(field))
            for field, count in visited_fields.items()
        ],
        reverse=True,
    )
    return "".join(f"{index:02d}" for percentage, field, index in results[:3])


if __name__ == "__main__":
    import runner

    runner.run(globals())

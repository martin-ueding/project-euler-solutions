import collections
import itertools
from typing import Iterator, Optional


def iter_triangular() -> Iterator[int]:
    for n in itertools.count(1):
        yield n * (n + 1) // 2


def iter_square() -> Iterator[int]:
    for n in itertools.count(1):
        yield n**2


def iter_pentagonal() -> Iterator[int]:
    for n in itertools.count(1):
        yield n * (3 * n - 1) // 2


def iter_hexagonal() -> Iterator[int]:
    for n in itertools.count(1):
        yield n * (2 * n - 1)


def iter_heptagonal() -> Iterator[int]:
    for n in itertools.count(1):
        yield n * (5 * n - 3) // 2


def iter_octogonal() -> Iterator[int]:
    for n in itertools.count(1):
        yield n * (3 * n - 2)


def generate_numbers(iterator: Iterator[int]) -> list[int]:
    result = []
    for number in iterator:
        if number >= 1000:
            result.append(number)
        if number >= 10000:
            break
    return result


def split_numbers(numbers: list[int]) -> dict[str, list[str]]:
    result = collections.defaultdict(list)
    for number in numbers:
        s = str(number)
        prefix = s[:2]
        suffix = s[2:]
        result[prefix].append(suffix)
    return result


def solution() -> int:
    all_numbers = [
        generate_numbers(iterator)
        for iterator in list(reversed([
            iter_triangular(),
            iter_square(),
            iter_pentagonal(),
            iter_hexagonal(),
            iter_heptagonal(),
            iter_octogonal(),
        ]))
    ]

    splits = [split_numbers(numbers) for numbers in all_numbers]
    for ss in itertools.permutations(splits[1:]):
        for prefix, suffixes in splits[0].items():
            for suffix in suffixes:
                if result := recursion([prefix + suffix], ss):
                    return sum(map(int, result))


def recursion(
    set_so_far: list[str], splits: list[dict[str, list[str]]]
) -> Optional[list[str]]:
    prefix = set_so_far[-1][2:]
    if splits:
        for candidate in splits[0].get(prefix, []):
            set_so_far.append(prefix + candidate)
            result = recursion(set_so_far, splits[1:])
            if result:
                return result
            set_so_far.pop()
    else:
        if prefix == set_so_far[0][:2]:
            return set_so_far


if __name__ == "__main__":
    import runner

    runner.run(globals())

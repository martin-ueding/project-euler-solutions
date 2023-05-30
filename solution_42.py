import json
import math


def read_words() -> list[str]:
    with open("data/p042_words.txt") as f:
        content = f.read()
    return json.loads(f"[{content}]")


def word_to_number(word: str) -> int:
    return sum(ord(c) - ord("A") + 1 for c in word)


def is_triangle_number(number: int) -> bool:
    s = int(round(math.sqrt(1 + 8 * number)))
    return s**2 == 1 + 8 * number and (-1 + s) % 2 == 0


def solution() -> int:
    return len(list(filter(is_triangle_number, map(word_to_number, read_words()))))


if __name__ == "__main__":
    import runner

    runner.run(globals())

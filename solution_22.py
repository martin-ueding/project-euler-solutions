import json


def solution() -> int:
    with open("data/p022_names.txt") as f:
        names = json.loads(f"[{f.read()}]")
    names.sort()
    scores = [sum(map(lambda c: ord(c) - ord("A") + 1, name)) for name in names]
    return sum(row * score for row, score in enumerate(scores, 1))


if __name__ == "__main__":
    import runner

    runner.run(globals())

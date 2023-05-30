from typing import Iterator


def iter_samples() -> Iterator[str]:
    with open("data/p079_keylog.txt") as f:
        for line in f:
            sample = line.strip()
            if sample:
                yield sample


def solution() -> None:
    samples = sorted(set(iter_samples()))
    chars = set(char for sample in samples for char in sample)
    previous_char = {char: set() for char in chars}
    for sample in samples:
        for i in range(2):
            previous_char[sample[i + 1]].add(sample[i])
    result = ""
    while previous_char:
        for char, before in previous_char.items():
            if not before:
                result += char
                break
        else:
            assert False
        del previous_char[char]
        for before in previous_char.values():
            if char in before:
                before.remove(char)
    return int(result)


if __name__ == "__main__":
    import runner

    runner.run(globals())

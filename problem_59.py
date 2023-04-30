import collections
import itertools
import json
import string


def read_file() -> list[int]:
    with open("data/p059_cipher.txt") as f:
        return json.loads(f"[{f.read()}]")


acceptable = set(
    ord(x) for x in string.ascii_letters + string.digits + " .,:;+?!/[]()'\""
)


def possible_keys(ciphertext: list[int]) -> set[int]:
    result = []
    for char in string.ascii_lowercase:
        plaintext = {ct ^ ord(char) for ct in set(ciphertext)}
        extras = plaintext - acceptable
        if not extras:
            result.append(ord(char))
    return result


def solution() -> int:
    ciphertext = read_file()
    keys_1 = possible_keys(ciphertext[::3])
    keys_2 = possible_keys(ciphertext[1::3])
    keys_3 = possible_keys(ciphertext[2::3])
    assert len(keys_1) == 1
    assert len(keys_2) == 1
    assert len(keys_3) == 1
    return sum(
        c ^ k
        for c, k in zip(ciphertext, itertools.cycle([keys_1[0], keys_2[0], keys_3[0]]))
    )


if __name__ == "__main__":
    import runner

    runner.run(globals())

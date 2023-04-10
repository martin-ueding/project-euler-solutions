def is_palindrome(n: int) -> bool:
    s = str(n)
    return s == s[::-1]


def is_palindrome_base_2(n: int) -> bool:
    s = bin(n)[2:]
    return s == s[::-1]


def solution() -> int:
    return sum(filter(is_palindrome_base_2, filter(is_palindrome, range(1, 1_000_000))))


if __name__ == "__main__":
    import runner

    runner.run(globals())

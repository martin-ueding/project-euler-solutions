def is_palindrome(n: int) -> bool:
    s = str(n)
    return s == s[::-1]


def is_palindrome_base_2(n: int) -> bool:
    s = bin(n)[2:]
    return s == s[::-1]


def solution() -> int:
    return sum(filter(is_palindrome_base_2, filter(is_palindrome, range(1, 1_000_000))))


def solution_procedural() -> int:
    result = 0
    for candidate in range(1, 1_000_000):
        if not is_palindrome(candidate):
            continue
        if not is_palindrome_base_2(candidate):
            continue
        result += candidate
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

from problem_36 import is_palindrome


def reverse_and_add(number: int) -> int:
    return number + int(str(number)[::-1])


def is_lychrel(number: int) -> int:
    for iteration in range(50):
        number = reverse_and_add(number)
        if is_palindrome(number):
            return False
    else:
        return True


def solution() -> None:
    lychrel_numbers = list(filter(is_lychrel, range(1, 10_000)))
    return len(lychrel_numbers)


if __name__ == "__main__":
    import runner

    runner.run(globals())

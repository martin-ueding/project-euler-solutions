from solution_41 import is_prime


def solution() -> int:
    for first in range(1111, 9999 - 2 * 3330):
        second = first + 3330
        third = first + 6660
        if (
            sorted(str(first)) == sorted(str(second)) == sorted(str(third))
            and is_prime(first)
            and is_prime(second)
            and is_prime(third)
        ):
            result = int(f"{first}{second}{third}")
            if result != 148748178147:
                return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

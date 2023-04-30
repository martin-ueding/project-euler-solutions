import itertools

from problem_3 import prime_generator


def int_with_zero(number: str) -> int:
    while number[0] == "0":
        number = number[1:]
    return int(number)


def has_property(digits: int) -> bool:
    for prime, begin in zip(prime_generator(), range(1, len(digits) - 2)):
        sub_digits = digits[begin : begin + 3]
        sub = int_with_zero("".join(sub_digits))
        if sub % prime > 0:
            return False
    else:
        return True


def solution_all_permutations() -> int:
    results = []
    for digits in itertools.permutations(map(str, range(10))):
        if digits[0] == 0:
            continue
        if has_property(digits):
            number = int("".join(digits))
            results.append(number)
    print(sorted(results))
    return sum(results)


def solution_selective_permutations() -> int:
    results = []
    for m in range(999 // 17 + 1):
        number = f"{17 * m:03d}"
        if len(set(number)) != 3:
            continue
        digits_available = set("0123456789") - set(number)
        results.extend(another_digit(number, digits_available))
    return sum(results)


if __name__ == "__main__":
    import runner

    runner.run(globals())

from problem_33 import greatest_common_denominator


def find_next_smaller_fraction(
    target: tuple[int, int], ceiling: int
) -> tuple[int, int]:
    target_numerator, target_denominator = target
    candidates = []
    for denominator in range(1, ceiling + 1):
        numerator = bisect_numerator(target, denominator)
        if numerator * target_denominator < target_numerator * denominator:
            numerator, denominator = reduce_fraction(numerator, denominator)
            candidates.append((numerator / denominator, numerator, denominator))
    m = max(candidates)
    return m[1:]


def bisect_numerator(target: tuple[int, int], denominator: int) -> int:
    target_numerator, target_denominator = target
    lower = 0
    upper = denominator * target_numerator // target_denominator
    while lower < upper - 1:
        middle = (lower + upper) // 2 + 1
        if middle * target_denominator > target_numerator * denominator:
            upper = middle
        else:
            lower = middle
    return lower


def _solution() -> int:
    return find_next_smaller_fraction((3, 7), 1_000_000)[0]


def reduce_fraction(numerator: int, denominator: int) -> tuple[int, int]:
    gcd = greatest_common_denominator(numerator, denominator)
    numerator //= gcd
    denominator //= gcd
    return numerator, denominator


def solution_faster() -> int:
    ceiling = 1_000_000
    for difference_denominator in reversed(range(ceiling)):
        numerator = 3 * difference_denominator
        denominator = 7 * difference_denominator
        numerator -= 1
        numerator, denominator = reduce_fraction(numerator, denominator)
        if denominator <= ceiling:
            return numerator, denominator


if __name__ == "__main__":
    import runner

    runner.run(globals())

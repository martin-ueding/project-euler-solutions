def get_long_division_cycle_length(denominator: int) -> int:
    numerator = 10
    digits = []
    remainders = []
    cur = numerator
    while True:
        digit = cur // denominator
        cur = cur % denominator
        if cur in remainders:
            break
        digits.append(digit)
        remainders.append(cur)
        cur *= 10
        if cur == 0:
            return 0
        while cur < denominator:
            cur *= 10
            digits.append(0)
    while remainders[0] != cur:
        remainders.pop(0)
    return len(remainders)


def solution_long_division() -> int:
    cycle_lengths = {denominator: get_long_division_cycle_length(denominator) for denominator in range(1, 1000)}
    return sorted(cycle_lengths.items(), key=lambda item: item[1])[-1][0]


if __name__ == "__main__":

    import runner

    runner.run(globals())

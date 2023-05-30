def solution_naive() -> int:
    return (28433 * 2**7830457 + 1) % 10**10


def solution_modulus() -> int:
    divisor = 10**10
    number = 28433
    for i in range(7830457):
        number *= 2
        number %= divisor
    number += 1
    number %= divisor
    return number


if __name__ == "__main__":
    import runner

    runner.run(globals())

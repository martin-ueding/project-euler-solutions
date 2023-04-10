def solution_big_integers() -> int:
    return sum(number**number for number in range(1, 1001)) % 10**10


def solution_remainder() -> int:
    cutoff = 10**10
    result = 0
    for number in range(1, 1001):
        summand = 1
        for _ in range(number):
            summand = (summand * number) % cutoff
        result = (result + summand) % cutoff
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

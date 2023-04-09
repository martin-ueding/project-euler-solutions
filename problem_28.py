def solution() -> int:
    accumulator = 1
    number = 1
    for loop in range(1, 501):
        for side in range(4):
            number += 2 * loop
            accumulator += number
    return accumulator


if __name__ == "__main__":
    import runner

    runner.run(globals())

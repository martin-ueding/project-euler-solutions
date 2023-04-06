def solution_naive() -> int:
    sum_numbers = 0
    sum_squares = 0
    for i in range(101):
        sum_numbers += i
        sum_squares += i**2
    return sum_numbers**2 - sum_squares


if __name__ == "__main__":
    import runner

    runner.run(globals())
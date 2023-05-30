def solution_direct() -> int:
    digits = [int(char) for char in str(2**1000)]
    return sum(digits)


if __name__ == "__main__":
    import runner

    runner.run(globals())

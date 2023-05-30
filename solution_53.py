def solution() -> int:
    result = 0
    for n in range(1, 101):
        for r in range(1, n):
            coefficient = 1
            for factor in range(n, n - r, -1):
                coefficient *= factor
            for factor in range(2, r + 1):
                coefficient //= factor
            if coefficient > 1_000_000:
                result += 1
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

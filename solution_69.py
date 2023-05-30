from solution_3 import prime_generator


def solution() -> int:
    ceiling = 1000000
    result = 1
    for prime in prime_generator():
        if result * prime > ceiling:
            return result
        result *= prime


if __name__ == "__main__":
    import runner

    runner.run(globals())

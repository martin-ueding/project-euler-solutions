def solution() -> int:
    return len({a**b for a in range(2, 101) for b in range(2, 101)})


if __name__ == "__main__":
    import runner

    runner.run(globals())

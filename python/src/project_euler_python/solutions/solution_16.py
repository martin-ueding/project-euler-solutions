def solution_direct() -> int:
    digits = [int(char) for char in str(2**1000)]
    return sum(digits)


if __name__ == "__main__":
    import python.src.project_euler_python.runner as runner

    runner.run(globals())

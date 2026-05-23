from python.src.project_euler_python.solutions.solution_2 import fibonacci_generator


def solution_iterative() -> int:
    for i, f in enumerate(fibonacci_generator(), 2):
        if len(str(f)) >= 1000:
            return i


if __name__ == "__main__":
    import python.src.project_euler_python.runner as runner

    runner.run(globals())

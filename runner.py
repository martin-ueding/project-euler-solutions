import datetime


def make_timing(callable) -> float:
    runs = 1
    timing = datetime.timedelta(0)
    callable()
    while timing < datetime.timedelta(milliseconds=100):
        runs *= 2
        start = datetime.datetime.now()
        for _ in range(runs):
            callable()
        end = datetime.datetime.now()
        timing = end - start
    return timing.total_seconds() / runs * 1000


def run(module_dict=None) -> None:
    solutions = {
        key: value for key, value in module_dict.items() if key.startswith("solution")
    }

    for solution_name, solution in solutions.items():
        timing = make_timing(solution)
        print(f"{solution()} from {solution_name} took {timing:.3f} ms")


def main() -> None:
    import problem_3 as problem

    run(problem.__dict__)


if __name__ == "__main__":
    main()

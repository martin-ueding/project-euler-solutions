import datetime


def make_timing(callable) -> float:
    runs = 1
    start = datetime.datetime.now()
    result = callable()
    end = datetime.datetime.now()
    timing = end - start
    while timing < datetime.timedelta(milliseconds=100):
        runs *= 2
        start = datetime.datetime.now()
        for _ in range(runs):
            callable()
        end = datetime.datetime.now()
        timing = end - start
    return result, timing.total_seconds() / runs * 1000


def run(module_dict=None) -> None:
    solutions = {
        key: value for key, value in module_dict.items() if key.startswith("solution")
    }

    for solution_name, solution in solutions.items():
        result, timing = make_timing(solution)
        print(f"{result} from {solution_name} took {timing:.3f} ms")


def main() -> None:
    import problem_3 as problem

    run(problem.__dict__)


if __name__ == "__main__":
    main()

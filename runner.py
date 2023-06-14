import datetime


def format_timing(seconds: float) -> str:
    if seconds > 60:
        m = int(seconds) // 60
        s = int(seconds) % 60
        return f"{m}:{s:02d} min"
    elif seconds > 10:
        return f"{int(seconds):d} s"
    elif seconds > 1:
        return f"{seconds:.1f} s"
    elif seconds > 0.010:
        return f"{int(seconds*1000)} ms"
    elif seconds > 0.001:
        return f"{seconds*1000:.1f} ms"
    elif seconds > 0.000_010:
        return f"{int(seconds*1000_000)} µs"
    elif seconds > 0.000_001:
        return f"{seconds*1000_000:.1f} ms"
    elif seconds > 0.000_000_010:
        return f"{int(seconds*1000_000_000)} ns"


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
    return result, timing.total_seconds() / runs


def run(module_dict=None) -> None:
    solutions = {
        key: value for key, value in module_dict.items() if key.startswith("solution")
    }

    for solution_name, solution in solutions.items():
        result, timing = make_timing(solution)
        print(f"{result} from {solution_name} took {format_timing(timing)}")


def main() -> None:
    import problem_3 as problem

    run(problem.__dict__)


if __name__ == "__main__":
    main()

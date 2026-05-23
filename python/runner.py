import datetime
import importlib
import argparse

def format_timing(seconds: float) -> str:
    if seconds > 60:
        m = int(seconds) // 60
        s = int(seconds) % 60
        return f"{m}:{s:02d} min"
    elif seconds > 10:
        return f"{seconds:.0f} s"
    elif seconds > 1:
        return f"{seconds:.1f} s"
    elif seconds > 0.100:
        return f"{seconds*1000:.0f} ms"
    elif seconds > 0.010:
        return f"{seconds*1000:.1f} ms"
    elif seconds > 0.001:
        return f"{seconds*1000:.2f} ms"
    elif seconds > 0.000_010:
        return f"{int(seconds*1000_000)} µs"
    elif seconds > 0.000_001:
        return f"{seconds*1000_000:.1f} ms"
    elif seconds > 0.000_000_010:
        return f"{int(seconds*1000_000_000)} ns"


def make_timing(callable) -> None:
    benchmark_start = datetime.datetime.now()
    timings_s = []
    result = None
    while datetime.datetime.now() - benchmark_start < datetime.timedelta(seconds=1) and len(timings_s) < 100:
        begin = datetime.datetime.now()
        result = callable()
        end = datetime.datetime.now()
        timings_s.append((end - begin).total_seconds())
    timings_s.sort()
    p0 = format_timing(timings_s[0])
    p25 = format_timing(timings_s[len(timings_s) // 4])
    p50 =  format_timing(timings_s[len(timings_s) // 2])
    p75 = format_timing(timings_s[len(timings_s) *3 // 4])
    p100 = format_timing(timings_s[-1])
    print(f"Solution: {result}")
    print(f"Timings: {p0} | {p25} | {p50} | {p75} | {p100} | {len(timings_s)} iterations")


def run(module_dict=None) -> None:
    solutions = {
        key: value for key, value in module_dict.items() if key.startswith("solution")
    }

    print("Language: Python")

    for solution_name, solution in solutions.items():
        print(f"Implementation: {solution_name}")
        make_timing(solution)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('problem_id', type=int)
    options = parser.parse_args()
    problem_id: int = options.problem_id
    print(f"Problem: {problem_id}")

    solution_x = importlib.import_module(f"solution_{problem_id}")
    
    run(solution_x.__dict__)


if __name__ == "__main__":
    main()

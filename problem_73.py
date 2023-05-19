from tqdm import tqdm
from problem_33 import greatest_common_denominator
from problem_71 import reduce_fraction


def solution_count_reduced() -> None:
    result = 0
    for denominator in tqdm(range(1, 12_000 + 1)):
        for numerator in range(1, denominator):
            if 1 / 3 < numerator / denominator < 1 / 2:
                if greatest_common_denominator(numerator, denominator) == 1:
                    result += 1
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())

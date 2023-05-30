import numpy as np

from solution_21 import get_all_divisors


def solution() -> int:
    abundant_numbers = [
        number for number in range(1, 28125) if sum(get_all_divisors(number)) > number
    ]

    an_vector = np.array(abundant_numbers)
    an_matrix = np.broadcast_to(
        an_vector, (len(abundant_numbers), len(abundant_numbers))
    )
    sums = an_matrix + an_matrix.T

    is_viable = np.ones((28125,))
    not_viable = np.array([s for s in set(sums.flatten()) if s < 28125])
    is_viable[not_viable] = 0
    return sum(np.arange(len(is_viable))[is_viable == 1].tolist())


if __name__ == "__main__":
    import runner

    runner.run(globals())

import itertools

from python.src.project_euler_python.solutions.solution_57 import square_root_sequence


def test_square_root_sequence() -> None:
    elems = list(itertools.islice(square_root_sequence(), 4))
    assert elems == [(3, 2), (7, 5), (17, 12), (41, 29)]

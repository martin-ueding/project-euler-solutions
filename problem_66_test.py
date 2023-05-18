from problem_66 import minimal_solution


def test_minimal_solution() -> None:
    assert minimal_solution(2) == 3
    assert minimal_solution(3) == 2
    assert minimal_solution(5) == 9
    assert minimal_solution(6) == 5
    assert minimal_solution(7) == 8

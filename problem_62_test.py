from problem_62 import CubeSet


def test_cube_set() -> None:
    cube_set = CubeSet()
    assert cube_set.is_cube(41063625)
    assert cube_set.is_cube(56623104)
    assert cube_set.is_cube(66430125)

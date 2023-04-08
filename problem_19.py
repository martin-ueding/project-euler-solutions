import datetime
from typing import Tuple


def solution_date_class() -> int:
    start = datetime.date(1901, 1, 1)
    end = datetime.date(2000, 12, 31)
    cur = start
    num_sundays = 0
    while cur <= end:
        if cur.weekday() == 6 and cur.day == 1:
            num_sundays += 1
        cur += datetime.timedelta(days=1)
    return num_sundays


Date = Tuple[int, int, int]


def increment_day(date: Date) -> Date:
    year, month, day = date
    day += 1
    if (
        day > 31
        or (day > 30 and month in [4, 6, 9, 11])
        or (
            day > 29
            and month == 2
            and year % 4 == 0
            and (year % 100 != 0 or year % 400 == 0)
        )
        or (
            day > 28
            and month == 2
            and (year % 4 != 0 or (year % 100 == 0 and year % 400 != 0))
        )
    ):
        day = 1
        month += 1
    if month > 12:
        month = 1
        year += 1
    return year, month, day


def solution_manual() -> int:
    start = (1901, 1, 1)
    end = (2000, 12, 31)
    cur_date = start
    cur_weekday = 1
    num_sundays = 0
    while cur_date <= end:
        if cur_weekday == 6 and cur_date[2] == 1:
            num_sundays += 1
        cur_date = increment_day(cur_date)
        cur_weekday = (cur_weekday + 1) % 7
    return num_sundays


if __name__ == "__main__":
    import runner

    runner.run(globals())

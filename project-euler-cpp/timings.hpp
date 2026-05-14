#pragma once

#include <chrono>

std::chrono::time_point<std::chrono::high_resolution_clock> now();

void report_duration(
    std::chrono::time_point<std::chrono::high_resolution_clock> const &start);

void report_solution(int const solution);
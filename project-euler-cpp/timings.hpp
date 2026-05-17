#pragma once

#include <chrono>
#include <functional>

std::chrono::time_point<std::chrono::high_resolution_clock> now();

void report_duration(
    std::chrono::time_point<std::chrono::high_resolution_clock> const &start);

void report_solution(int const solution);


void run_solution(std::function<int(void)> const &solution);
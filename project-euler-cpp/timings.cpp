#include "timings.hpp"

#include <iostream>

std::chrono::time_point<std::chrono::high_resolution_clock> now() {
  return std::chrono::high_resolution_clock::now();
}

void report_duration(
    std::chrono::time_point<std::chrono::high_resolution_clock> const &start) {
  auto const end = std::chrono::high_resolution_clock::now();
  std::chrono::duration<double, std::milli> elapsed_ms = end - start;
  std::chrono::duration<double, std::micro> elapsed_us = end - start;
  std::chrono::duration<double, std::nano> elapsed_ns = end - start;
  if (elapsed_ms.count() > 1) {
    std::cout << "Duration: " << elapsed_ms.count() << " ms" << std::endl;
  } else if (elapsed_us.count() > 1) {
    std::cout << "Duration: " << elapsed_us.count() << " µs" << std::endl;
  } else if (elapsed_ns.count() > 1) {
    std::cout << "Duration: " << elapsed_ns.count() << " ns" << std::endl;
  }
}

void report_solution(int const solution) {
  std::cout << "Solution: " << solution << std::endl;
}
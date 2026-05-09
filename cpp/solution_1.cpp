#include <chrono>
#include <iostream>

#include "timings.hpp"

int main() {
  auto const start = std::chrono::high_resolution_clock::now();
  int sum = 0;
  for (int i = 0; i < 1000; ++i) {
    if (i % 3 == 0 || i % 5 == 0) {
      sum += i;
    }
  }
  report_duration(start);
  report_solution(sum);
}
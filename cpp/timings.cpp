#include "timings.hpp"

#include <iostream>
#include <sstream>

std::chrono::time_point<std::chrono::high_resolution_clock> now() {
    return std::chrono::high_resolution_clock::now();
}

void report_duration(
    std::chrono::time_point<std::chrono::high_resolution_clock> const& start) {
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

std::string format_duration(double const duration_ms) {
    std::stringstream ss;
    if (duration_ms > 1000) {
        ss << duration_ms / 1000 << " s";
    } else if (duration_ms > 1) {
        ss << duration_ms << " ms";
    } else if (duration_ms > 0.001) {
        ss << duration_ms * 1000 << " µs";
    } else if (duration_ms > 0.000001) {
        ss << duration_ms * 1000000 << " ns";
    }
    return ss.str();
}

int find_batch_size(std::function<int64_t(void)> const& solution) {
    int batch_size = 1;

    while (true) {
        auto benchmark_start = std::chrono::high_resolution_clock::now();
        for (int i = 0; i < batch_size; ++i) {
            solution();
        }
        auto const elapsed =
            std::chrono::high_resolution_clock::now() - benchmark_start;
        double const seconds = std::chrono::duration<double>(elapsed).count();

        if (seconds < 0.001) {
            batch_size *= 10;
        } else {
            break;
        }
    }
    std::cout << "Batch Size: " << batch_size << std::endl;

    return batch_size;
}

void run_solution(std::function<int64_t(void)> const& solution) {
    auto const batch_size = find_batch_size(solution);
    auto const begin_benchmark = std::chrono::high_resolution_clock::now();
    std::vector<double> timings_ms;
    int64_t result;
    while (static_cast<std::chrono::duration<double, std::milli>>(
               std::chrono::high_resolution_clock::now() - begin_benchmark)
                   .count() < 1000 &&
           timings_ms.size() < 100) {
        auto const begin = std::chrono::high_resolution_clock::now();
        for (int i = 0; i < batch_size; ++i) {
            result = solution();
        }
        auto const end = std::chrono::high_resolution_clock::now();
        timings_ms.push_back(
            static_cast<std::chrono::duration<double, std::milli>>(end - begin)
                .count() /
            batch_size);
    }
    std::sort(timings_ms.begin(), timings_ms.end());

    std::cout << "Solution: " << result << std::endl;
    std::cout << "Timings: " << format_duration(timings_ms.front()) << " | "
              << format_duration(timings_ms.at(timings_ms.size() / 4)) << " | "
              << format_duration(timings_ms.at(timings_ms.size() / 2)) << " | "
              << format_duration(timings_ms.at(timings_ms.size() * 3 / 4))
              << " | " << format_duration(timings_ms.back()) << " | "
              << timings_ms.size() << " iterations" << std::endl;
}
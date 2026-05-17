cmake_minimum_required(VERSION 4.3.0)

project(project_euler_solutions CXX)

add_library(helpers primes.hpp primes.cpp timings.hpp timings.cpp)

add_executable(solution_1 solution_1.cpp)
target_link_libraries(solution_1 PRIVATE helpers)

add_executable(solution_3 solution_3.cpp)
target_link_libraries(solution_3 PRIVATE helpers)
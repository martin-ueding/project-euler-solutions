cmake_minimum_required(VERSION 4.3.0)

project(project_euler_solutions CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_executable(main
    main.cpp
    registry.hpp registry.cpp
    primes.hpp primes.cpp
    timings.hpp timings.cpp

    solution_1.cpp
    solution_3.cpp
)
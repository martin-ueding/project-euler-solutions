#include <iostream>

#include "registry.hpp"
#include "timings.hpp"

int main(int const argc, char const* const* const argv) {
    int const id = std::atoi(argv[1]);

    std::cout << "Problem: " << id << std::endl;
    std::cout << "Language: C++" << std::endl;

    for (auto const& [name, callable] : solutions.at(id)) {
        std::cout << "Implementation: " << name << std::endl;
        run_solution(callable);
    }
}
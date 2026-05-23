#pragma once

#include <functional>
#include <map>
#include <cstdint>

extern std::map<int, std::function<int64_t(void)>> solutions;

class Registration {
   public:
    Registration(int const id, std::function<int64_t(void)> solution) {
        solutions[id] = solution;
    }
};
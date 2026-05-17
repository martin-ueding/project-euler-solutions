#pragma once

#include <functional>
#include <map>

extern std::map<int, std::function<int(void)>> solutions;

class Registration {
   public:
    Registration(int const id, std::function<int(void)> solution) {
        solutions[id] = solution;
    }
};
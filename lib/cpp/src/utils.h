#ifndef UTILS_H
#define UTILS_H

#include <algorithm>
#include <ostream>
#include <variant>
#include <vector>

template <typename T>
std::ostream& operator<<(std::ostream& os, const std::vector<T>& value) {
    os << "[";
    for (auto el : value) os << el << ",";
    os << "\b]";

    return os;
}

#endif

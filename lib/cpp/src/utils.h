#ifndef UTILS_H
#define UTILS_H

#include <algorithm>
#include <ostream>
#include <variant>
#include <vector>

template <typename T>
bool vectorEqualityWithoutOrdering(std::vector<T>& a, std::vector<T>& b) {
    if (a.size() != b.size()) return false;

    std::sort(a.begin(), a.end());
    std::sort(b.begin(), b.end());

    return a == b;
}

template <typename T>
std::ostream& operator<<(std::ostream& os, const std::vector<T>& value) {
    os << "[";
    for (auto el : value) os << el << ",";
    os << "\b]";

    return os;
}

#endif

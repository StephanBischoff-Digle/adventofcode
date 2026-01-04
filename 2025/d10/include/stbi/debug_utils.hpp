#pragma once

#include <algorithm>
#include <iostream>
#include <span>

namespace stbi::debug {

    template <typename T>
    void print_vec(std::span<const T> vec, std::span<const T> filter) {
        std::cout << "[ ";
        for (size_t i{ 0 }; i < vec.size() - 1; i++) {
            if (!std::ranges::contains(filter, vec[i])) {
                std::cout << vec[i] << ", ";
            } else {
                std::cout << "x, ";
            }
        }

        if (!std::ranges::contains(filter, vec[vec.size() - 1])) {
            std::cout << vec[vec.size() - 1];
        } else {
            std::cout << "x";
        }
        std::cout << " ]";
    }
}  // namespace stbi::debug

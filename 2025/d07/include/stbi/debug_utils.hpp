#pragma once

#include <iostream>
#include <span>

namespace stbi::debug {

    template <typename T>
    void print_vec(std::span<T> vec) {
        std::cout << "[ ";
        for (size_t i{ 0 }; i < vec.size() - 1; i++) {
            if (vec[i] > 0) {
                std::cout << vec[i] << ", ";
            } else {
                std::cout << " , ";
            }
        }

        if (vec[vec.size() - 1] > 0) {
            std::cout << vec[vec.size() - 1];
        } else {
            std::cout << " ";
        }
        std::cout << " ]";
    }
}  // namespace stbi::debug

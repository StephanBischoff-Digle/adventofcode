cmake_minimum_required(VERSION 3.17..4.1)
project(aoc25_1 VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
add_compile_options(
    -Wall
    -Wextra
    -Wpedantic
    -Werror
    -Wconversion
    -Wsign-conversion
)

add_executable(aoc25_1_a src/main_1.cpp)
set_target_properties(aoc25_1_a PROPERTIES INTERPROCEDURAL_OPTIMIZATION ON)

add_executable(aoc25_1_b src/main_2.cpp)
set_target_properties(aoc25_1_b PROPERTIES INTERPROCEDURAL_OPTIMIZATION ON)

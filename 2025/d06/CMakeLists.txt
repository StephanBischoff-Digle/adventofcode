cmake_minimum_required(VERSION 3.17..4.1)
project(aoc25_6 VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

function(set_project_warnings target)
    target_compile_options(${target} PRIVATE
        -Wall
        -Wextra
        -Wpedantic
        -Werror
        -Wconversion
        -Wsign-conversion
    )
endfunction()

# include(FetchContent)
# FetchContent_Declare(
#     Catch2
#     GIT_REPOSITORY https://github.com/catchorg/Catch2.git
#     GIT_TAG v3.11.0)
# FetchContent_MakeAvailable(Catch2)
#
# ==================================================

add_executable(aoc25_6_a src/main_1.cpp)
target_include_directories(aoc25_6_a PRIVATE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>)
set_project_warnings(aoc25_6_a)
set_target_properties(aoc25_6_a PROPERTIES INTERPROCEDURAL_OPTIMIZATION ON)

# ==================================================

add_executable(aoc25_6_b src/main_2.cpp)
target_include_directories(aoc25_6_b PRIVATE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>)
set_project_warnings(aoc25_6_b)
set_target_properties(aoc25_6_b PROPERTIES INTERPROCEDURAL_OPTIMIZATION ON)

# ==================================================

# add_executable(tests tests/tests.cpp)
# target_include_directories(tests PRIVATE
#     $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
#     $<INSTALL_INTERFACE:include>)
# target_include_directories(tests SYSTEM PRIVATE $<TARGET_PROPERTY:Catch2::Catch2WithMain,INTERFACE_INCLUDE_DIRECTORIES>)
# set_project_warnings(tests)
# target_link_libraries(tests PRIVATE Catch2::Catch2WithMain)
# set_target_properties(tests PROPERTIES INTERPROCEDURAL_OPTIMIZATION ON)
#
# include(CTest)
# include(Catch)
# catch_discover_tests(tests)

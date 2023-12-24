cmake_minimum_required(VERSION 3.27)

set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_CXX_STANDARD 20)

project(FiveLetters)

include(CTest) # We include CTest which is part of CMake

add_subdirectory(lib)

# We check if this is the main file
# you don't usually want users of your library to
# execute tests as part of their build
if (${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_SOURCE_DIR})
    add_subdirectory(tests)
endif ()
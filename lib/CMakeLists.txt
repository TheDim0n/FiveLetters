cmake_minimum_required(VERSION 3.27)

project(FiveLetters)

add_library(FiveLetters ./src)

include_directories(include)

target_sources(
    ${PROJECT_NAME}
    PRIVATE
        src/files.cpp
        src/game.cpp
)

target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
)
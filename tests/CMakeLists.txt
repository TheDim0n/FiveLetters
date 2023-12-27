cmake_minimum_required(VERSION 3.27)

project(Tests)


file(GLOB MOCK_FILES ../mock/*)

file(COPY ${MOCK_FILES} DESTINATION ./mock)

add_executable(TestGame game.cpp)
add_executable(TestFiles files.cpp)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/../lib/include
)

target_link_libraries(TestGame PRIVATE FiveLetters)
target_link_libraries(TestFiles PRIVATE FiveLetters)

enable_testing()

add_test(
    NAME TestGame
    COMMAND TestGame
)

add_test(
    NAME TestFiles
    COMMAND TestFiles
)
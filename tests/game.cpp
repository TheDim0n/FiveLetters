#include <iostream>
#include <cassert>

#include <game.h>


void test_game() {
    const std::string solution = "право";
    const std::string bad_solution = "народ";
    const uint8_t attemptions = 5;

    Game::Game game = Game::Game(solution, attemptions);
    bool status = game.check_attemption(solution);
    assert(status == true);

    status = game.check_attemption(bad_solution);
    assert(status == false);

    assert(game.get_attemption() == attemptions - 2);
}

int main() {
    test_game();
    return EXIT_SUCCESS;
}
#include <iostream>
#include <cassert>

#include <game.h>


void test_game() {
    const std::string solution = "фывап";
    const std::string badSolution = "йавыф";
    const uint8_t attemptions = 5;

    std::array<constants::State, 5> correctState = {
        constants::State::inCorrectPos,
        constants::State::inCorrectPos,
        constants::State::inCorrectPos,
        constants::State::inCorrectPos,
        constants::State::inCorrectPos,
    };

    std::array<constants::State, 5> badState = {
        constants::State::notFound,
        constants::State::inAnoterPos,
        constants::State::inCorrectPos,
        constants::State::inAnoterPos,
        constants::State::inAnoterPos,
    };

    Game::Game game = Game::Game(solution, attemptions);
    bool status = game.checkAttemption(solution);
    assert(status == true);
    assert(correctState == game.getStates());
    assert(badState != game.getStates());

    status = game.checkAttemption(badSolution);
    assert(status == false);
    assert(correctState != game.getStates());
    assert(badState == game.getStates());

    assert(game.getAttemption() == attemptions - 2);
}

int main() {
    test_game();
    return EXIT_SUCCESS;
}
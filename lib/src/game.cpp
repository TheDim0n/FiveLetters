#include <game.h>


Game::Game::Game(std::string solution, uint8_t attemptions) {
    this->solution = solution;
    this->attemptions = attemptions;
    this->word_size = solution.size();
    this->states = {
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined
    };
}

const bool Game::Game::checkAttemption(const std::string& attemption_word) {
    bool status = true;
    for (size_t i = 1; i < solution.size(); i += 2) {
        const size_t stateIndex = i / 2;
        const bool isEquial = (
            this->solution[i-1] == attemption_word[i-1]
            && this->solution[i] == attemption_word[i]
        );
        if (isEquial) this->states[stateIndex] = constants::State::inCorrectPos;
        else {
            status = false;
            size_t index = this->solution.find(attemption_word[i]);
            if (index == std::string::npos) this->states[stateIndex] = constants::State::notFound;
            else this->states[stateIndex] = constants::State::inAnoterPos;
        }
    }
    this->attemptions--;
    return status;
}

const std::array<constants::State, 5>& Game::Game::getStates() {
    return this->states;
};

const uint8_t& Game::Game::getAttemption() {
    return this->attemptions;
}
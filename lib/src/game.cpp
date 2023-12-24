#include <game.h>


Game::Game::Game(std::string solution, uint8_t attemptions) {
    this->solution = solution;
    this->attemptions = attemptions;
    this->word_size = solution.size();
    this->states = {0, 0, 0, 0, 0};
}

const bool Game::Game::check_attemption(const std::string& attemption_word) {
    bool status = true;
    for (size_t i = 1; i < solution.size(); i += 2) {
        const size_t stateIndex = i / 2;
        const bool isEquial = (
            this->solution[i-1] == attemption_word[i-1]
            && this->solution[i] == attemption_word[i]
        );
        if (isEquial) this->states[stateIndex] = 1;
        else {
            status = false;
            size_t index = this->solution.find(attemption_word[i]);
            if (index == std::string::npos) this->states[stateIndex] = 3;
            else this->states[stateIndex] = 2;
        }
    }
    this->attemptions--;
    return status;
}

const std::vector<uint8_t>& Game::Game::get_states() {
    return this->states;
};

const uint8_t& Game::Game::get_attemption() {
    return this->attemptions;
}
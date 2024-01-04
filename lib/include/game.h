#ifndef GAME_H_
#define GAME_H_

#ifdef __linux
#include <cstdint>
#endif

#include <array>
#include <string>

#include <constants.h>


namespace Game {
class Game {
private:
    std::string solution;
    std::array<constants::State, 5> states;
    size_t word_size;
    uint8_t attemptions;

public:
    Game(std::string solution, uint8_t attemptions);

    const bool checkAttemption(const std::string& attemption_word);
    const std::array<constants::State, 5>& getStates();
    const uint8_t& getAttemption();
};
}
#endif

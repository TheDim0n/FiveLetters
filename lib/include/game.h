#ifndef GAME_H_
#define GAME_H_

#include <string>
#include <vector>


namespace Game {
class Game {
private:
    std::string solution;
    std::vector<uint8_t> states; // TODO: use std::array<constants::State, 5>
    size_t word_size;
    uint8_t attemptions;

public:
    Game(std::string solution, uint8_t attemptions);

    const bool check_attemption(const std::string& attemption_word);
    const std::vector<uint8_t>& get_states();
    const uint8_t& get_attemption();
};
}
#endif

#ifndef FILES_H_
#define FILES_H_

#ifdef __linux
#include <cstdint>
#endif

#include <array>
#include <fstream>
#include <map>
#include <optional>
#include <string>
#include <vector>
#include <ctime>

#include <constants.h>


namespace files {
class ResolvedFile {
private:
    std::string filePath;

public:
    std::vector<unsigned long> resolves;
    ResolvedFile(const std::string& filePath);
    void updateResolved();
    void clear();
};


class WordsFile {
private:
    std::map<unsigned long, std::string> words;
    std::string filePath;
    char sep;

    std::pair<unsigned long, std::string> getWordFromString(
        const std::string& line, char sep
    );

public:
    WordsFile(const std::string& filePath, char sep = ';');
    std::pair<unsigned long, std::string> getRandomUnresolvedWord(
        const std::vector<unsigned long>& resolved
    );
    std::string getWordByID(const unsigned long id);

};

class StateFile {
private:
    std::string filePath;
    std::array<constants::State, 5> states;
    unsigned long wordID;
    uint8_t attemptions;

public:
    StateFile(const std::string& filePath);
    unsigned long getWordID();
    std::array<constants::State, 5> getStates();
    uint8_t getAttemptions();
    void update(
        unsigned long wordID,
        std::optional<std::array<constants::State, 5>> states = std::nullopt,
        std::optional<uint8_t> attemptions = std::nullopt
    );
};
}
#endif

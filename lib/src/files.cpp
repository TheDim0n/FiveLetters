#include <iostream>
#include <set>
#include <sstream>

#include <files.h>


files::ResolvedFile::ResolvedFile(const std::string& filePath) {
    this->filePath = filePath;

    std::ifstream input(filePath);
    std::string line;

    if (input.is_open()) {
        while (std::getline(input, line)) {
            this->resolves.push_back(std::stoul(line));
        }
        input.close();
    } else {
        throw std::runtime_error("File " + filePath + " not found\n");
    }
}

void files::ResolvedFile::updateResolved() {
    std::ofstream out(this->filePath);

    if (out.is_open()) {
        for (auto resolved : this->resolves) out << resolved << std::endl;
        out.close();
    } else {
        throw std::runtime_error("File " + filePath + " not found\n");
    }
}

void files::ResolvedFile::clear() {
    std::ofstream out(this->filePath);
    if (out.is_open()) out.close();
    else {
        throw std::runtime_error("File " + filePath + " not found\n");
    }
}

files::WordsFile::WordsFile(const std::string& filePath, char sep) {
    this->filePath = filePath;
    this->sep = sep;

    std::ifstream input(filePath);
    std::string line;

    if (input.is_open()) {
        while (std::getline(input, line)) {
            const auto word = this->getWordFromString(line, sep);
            this->words[word.first] = word.second;
        }
        input.close();
    }
}

std::pair<unsigned long, std::string> files::WordsFile::getWordFromString(
    const std::string& line, char sep
) {
    std::pair<unsigned long, std::string> word {};
    size_t sepPos = 0;
    size_t lineSize = line.size();
    while (sepPos < lineSize && line[sepPos] != sep) sepPos++;
    word.first = std::stoul(line.substr(0, sepPos));
    word.second = line.substr(sepPos, lineSize - sepPos);
    return word;
}

std::pair<unsigned long, std::string> files::WordsFile::getRandomUnresolvedWord(
    const std::vector<unsigned long>& resolved
) {
    std::set<unsigned long> wordIDs = {};
    for (auto& [id, _]: this->words) wordIDs.insert(id);
    for (auto& item: resolved) {
        if (wordIDs.contains(item)) wordIDs.erase(item);
    }

    std::srand(std::time(nullptr));
    unsigned long randomIndex = std::rand() % wordIDs.size();
    auto randomWordID = wordIDs.find(randomIndex);
    for (int i = 1; i < 10; i++) {
        if (randomWordID != wordIDs.end()) break;
        randomIndex = std::rand() % wordIDs.size();
        randomWordID = wordIDs.find(randomIndex);
    }
    return *this->words.find(*randomWordID);
}

std::string files::WordsFile::getWordByID(const unsigned long id) {
    return this->words.at(id);
}

files::StateFile::StateFile(const std::string& filePath) {
    this->filePath = filePath;

    std::ifstream input(filePath);
    std::string line;

    if (input.is_open()) {
        std::getline(input, line);
        this->wordID = std::stoul(line);
        if (this->wordID == 0) {
            this->states = {
                constants::State::undefined,
                constants::State::undefined,
                constants::State::undefined,
                constants::State::undefined,
                constants::State::undefined
            };
            this->attemptions = constants::attemptions;
        } else {
            size_t i = 0;
            std::getline(input, line);
            std::stringstream str(line);
            while (std::getline(str, line, ' ')) {
                this->states[i] = static_cast<constants::State>(std::stoi(line));
                i++;
            }
            std::getline(input, line);
            this->attemptions = static_cast<uint8_t>(std::stoi(line));
        }
        input.close();
    } else {
        throw std::runtime_error("File " + filePath + " not found\n");
    }
}

void files::StateFile::update(
    unsigned long wordID,
    std::optional<std::array<constants::State, 5>> states,
    std::optional<uint8_t> attemptions
) {
    this->wordID = wordID;
    this->states = states.value_or(std::array<constants::State, 5> {
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined,
        constants::State::undefined
    });
    this->attemptions = attemptions.value_or(constants::attemptions);

    std::ofstream out(this->filePath);

    if (out.is_open()) {
        out << this->wordID << std::endl;
        if (states) {
            const size_t len = this->states.size();
            for (size_t i = 0; i < len; i++) {
                out << this->states[i];
                if (i == len - 1) out<< std::endl;
                else out<< ' ';
            }
        }
        if (attemptions) out << static_cast<int>(this->attemptions) << std::endl;
        out.close();
    } else {
        throw std::runtime_error("File " + filePath + " not found\n");
    }
}

unsigned long files::StateFile::getWordID() {
    return this->wordID;
}

uint8_t files::StateFile::getAttemptions() {
    return this->attemptions;
}

std::array<constants::State, 5> files::StateFile::getStates() {
    return this->states;
}
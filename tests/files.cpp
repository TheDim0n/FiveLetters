#include <cassert>
#include <iostream>
#include <filesystem>
#include <optional>

#include <files.h>


namespace fs = std::filesystem;


void testResolvedFile() {

    const std::string resolvedFilePath = "./mock/resolved.txt";

    files::ResolvedFile resolvedfile = files::ResolvedFile(resolvedFilePath);
    assert(resolvedfile.resolves.size() == 0);
    resolvedfile.resolves.push_back(123);
    assert(resolvedfile.resolves.size() == 1);
    resolvedfile.updateResolved();

    resolvedfile = files::ResolvedFile(resolvedFilePath);
    assert(resolvedfile.resolves.size() == 1);
    resolvedfile.clear();

    resolvedfile = files::ResolvedFile(resolvedFilePath);
    assert(resolvedfile.resolves.size() == 0);
}

void testWordsFile() {
    const std::string wordsFilePath = "./mock/words.csv";

    auto wordsFile = files::WordsFile(wordsFilePath);

    std::vector<unsigned long> resolved = {};

    for(size_t i = 0; i < 1'000; i++) {
        const auto firstID = wordsFile.getRandomUnresolvedWord(resolved);
        resolved.push_back(firstID.first);
        const auto secondID = wordsFile.getRandomUnresolvedWord(resolved);
        assert(firstID.first != secondID.first);
        resolved.clear();
    }

    auto word = wordsFile.getWordByID(1);

    try {
        word = wordsFile.getWordByID(-100);
        std::terminate();
    } catch (const std::exception& e) {}
}


void testStateFile() {
    const std::string stateFilePath = "./mock/state.txt";
    auto stateFile = files::StateFile(stateFilePath);
    assert(stateFile.getWordID() == 0);
    assert(stateFile.getAttemptions() == constants::attemptions);
    for (const auto state: stateFile.getStates()) {
        assert(state == constants::State::undefined);
    }

    const std::array<constants::State, 5> states = {
        constants::State::notFound,
        constants::State::notFound,
        constants::State::notFound,
        constants::State::notFound,
        constants::State::notFound
    };

    stateFile.update(1, states, 3);

    stateFile = files::StateFile(stateFilePath);

    assert(stateFile.getWordID() == 1);
    assert(stateFile.getAttemptions() == static_cast<uint8_t>(3));
    for (const auto state: stateFile.getStates()) {
        assert(state == constants::State::notFound);
    }

    stateFile.update(0);

    assert(stateFile.getWordID() == 0);
    assert(stateFile.getAttemptions() == constants::attemptions);
    for (const auto state: stateFile.getStates()) {
        assert(state == constants::State::undefined);
    }
}


int main() {
    testResolvedFile();
    testWordsFile();
    testStateFile();
    return EXIT_SUCCESS;
}
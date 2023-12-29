#include <cassert>
#include <iostream>
#include <filesystem>

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

    for(size_t i = 0; i < 100; i++) {
        const auto firstID = wordsFile.getRandomUnresolvedWord(resolved);
        resolved.push_back(firstID.first);
        const auto secondID = wordsFile.getRandomUnresolvedWord(resolved);
        assert(firstID.first != secondID.first);
        resolved.clear();
    }
}


int main() {
    testResolvedFile();
    return EXIT_SUCCESS;
}
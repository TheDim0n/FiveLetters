#include <iostream>

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

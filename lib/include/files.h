#ifndef FILES_H_
#define FILES_H_

#include <string>
#include <vector>
#include <fstream>
#include <map>


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
    const std::string& getRandomUnresolvedWord(
        const std::vector<unsigned long>& resolved
    );

};
}
#endif

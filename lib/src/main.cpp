#include <iostream>
#include <fstream>
#include <string>
#include <vector>


int main() {
    const std::string FILE_PATH = "./mock/words.csv";
    // const std::string OUTPUT = "./mock/words.csv";

    std::string line;
    std::ifstream out(FILE_PATH);
    // std::ofstream in(OUTPUT);
    std::vector<std::string> words{};

    if (out.is_open())
    {
        while (std::getline(out, line, ';'))
        {
            std::cout<< line << std::endl;
        }
        out.close();
    }
    std::cout<< words.size() << std::endl;
    // std::cout<< count;
}

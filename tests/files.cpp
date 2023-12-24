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


int main() {
    testResolvedFile();
    return EXIT_SUCCESS;
}
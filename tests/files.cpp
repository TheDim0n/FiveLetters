#include <cassert>
#include <iostream>

#include <files.h>


int main() {
    const std::string resolvedFilePath = "../mock/resolved.txt";
    files::ResolvedFile resolvedfile = files::ResolvedFile(resolvedFilePath);
    assert(resolvedfile.resolves.size() == 0);
    resolvedfile.resolves.push_back(123);
    assert(resolvedfile.resolves.size() == 1);
    resolvedfile.updateResolved();
    return EXIT_SUCCESS;
}
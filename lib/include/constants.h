#ifndef CONSTANTS_H_
#define CONSTANTS_H_

#ifdef __linux
#include <cstdint>
#endif


namespace constants {
enum State {undefined, notFound, inAnoterPos, inCorrectPos};
const uint8_t attemptions = 5;
}

#endif
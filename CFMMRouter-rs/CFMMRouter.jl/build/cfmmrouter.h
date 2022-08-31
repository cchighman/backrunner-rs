// Julia headers (for initialization and gc commands)
#include "uv.h"
#include "julia.h"


typedef char cfmmChar;
typedef const cfmmChar* cfmmString;

// prototype of the C entry points in our application
int julia_cfmmrouter();
const char* route(const char* routes);

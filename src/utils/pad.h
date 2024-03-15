#pragma once

#define COMBINE_AUX(a, b) a##b
#define COMBINE(a, b) COMBINE_AUX(a, b)
#define PAD(offset) unsigned char COMBINE(pad, __LINE__)[offset];

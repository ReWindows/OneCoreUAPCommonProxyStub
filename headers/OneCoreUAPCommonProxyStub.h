// Windissect verified C++ surface for OneCoreUAPCommonProxyStub.dll.
// Candidate class declarations are isolated in the _candidates header.
// Validated dialects: C++17, C++20, C++23, and C++26 draft.
#pragma once

#if defined(_MSVC_LANG)
#  define WINDISSECT_CPLUSPLUS _MSVC_LANG
#else
#  define WINDISSECT_CPLUSPLUS __cplusplus
#endif
#if WINDISSECT_CPLUSPLUS < 201703L
#  error "Windissect C++ output requires C++17 or newer"
#endif

#include "windissect_abi.h"
#include "windissect_forwards.h"
#include "windissect_vcalls.h"
#include "windissect_calls.h"

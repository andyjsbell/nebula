#pragma once
#include "DXGIManager.h"

extern "C" bool CreateManager(int outWidth, int outHeight);

extern "C" bool IsSupported();

extern "C" bool GetOutputRect(int& x, int&y, int& width, int& height);

extern "C" bool GetOriginalRect(int& x, int&y, int& width, int& height);

extern "C" bool GetOutputBits(BYTE** pBits, DWORD& outLen, bool& nv12);

// Nebula.cpp : Defines the functions for the static library.
//

#include "pch.h"
#include "DXGIManager.h"
#include "Nebula.h"
extern "C" DXGIManager* gManager = nullptr;

extern "C" bool CreateManager(int outWidth, int outHeight)
{
	gManager = new DXGIManager(outWidth, outHeight);
	return true;
}

extern "C" bool IsSupported()
{
	if (gManager != nullptr)
	{
		return gManager->IsSupported() == S_OK;
	}

	return false;
}
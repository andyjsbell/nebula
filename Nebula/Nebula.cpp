#include "Nebula.h"
#include "DXGIManager.h"

DXGIManager * _manager = nullptr;

extern "C" bool CreateManager(int outWidth, int outHeight)
{
	if (_manager == nullptr)
		_manager = new DXGIManager;

	return true;
}

extern "C" bool IsSupported()
{
	if (_manager != nullptr)
	{
		return _manager->IsSupported() == S_OK;
	}

	return false;
}
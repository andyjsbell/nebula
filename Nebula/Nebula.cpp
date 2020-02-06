#include "Nebula.h"

DXGIManager * _manager = nullptr;

extern "C" bool CreateManager(int outWidth, int outHeight)
{
	if (_manager == nullptr)
	{
		CoInitialize(NULL);
		_manager = new DXGIManager(outWidth, outHeight);
	}
	HRESULT hr = _manager->SetCaptureSource(CaptureSource::CSDesktop);
	
	return hr == S_OK;
}

extern "C" bool IsSupported()
{
	if (_manager != nullptr)
	{
		return _manager->IsSupported() == S_OK;
	}

	return false;
}

extern "C" bool GetOutputRect(int& x, int&y, int& width, int& height)
{
	if (_manager != nullptr)
	{
		RECT r;
		if (S_OK == _manager->GetOutputRect(r))
		{
			x = r.left;
			y = r.top;
			width = r.right - r.left;
			height = r.bottom -  r.top;
			
			return true;
		}
	}

	return false;
}

extern "C" bool GetOriginalRect(int& x, int&y, int& width, int& height)
{
	if (_manager != nullptr)
	{
		RECT r;
		if (S_OK == _manager->GetOriginalRect(r))
		{
			x = r.left;
			y = r.top;
			width = r.right - r.left;
			height = r.bottom - r.top;

			return true;
		}
	}

	return false;
}

extern "C" bool GetOutputBits(BYTE** pBits, DWORD& outLen, bool& nv12)
{
	if (_manager != nullptr)
	{
		return S_OK == _manager->GetOutputBits(pBits, outLen, nv12);
	}

	return false;
}

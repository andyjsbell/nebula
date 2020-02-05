#pragma once
#define WIN32_LEAN_AND_MEAN             // Exclude rarely-used stuff from Windows headers
#include <windows.h>
#include <atlbase.h>
#include <DXGITYPE.h>
#include <DXGI1_2.h>
#include <d3d11.h>
#include <Wincodec.h>
#include <vector>
#include <comdef.h>
#include <mfapi.h>
#include <mfidl.h>
#include <mfreadwrite.h>
#include <wmcodecdsp.h>
#include <codecapi.h>
#include <mferror.h>
#include <evr.h>
#include <chrono>

#include <dxgi1_2.h>
#include <d3d11.h>
#include <mfreadwrite.h>
#include <Mferror.h>
#include <gdiplus.h>
#include <memory>
#include "Debug.h"

_COM_SMARTPTR_TYPEDEF(IDXGIFactory1, __uuidof(IDXGIFactory1));
_COM_SMARTPTR_TYPEDEF(ID3D11Device, __uuidof(ID3D11Device));
_COM_SMARTPTR_TYPEDEF(ID3D11DeviceContext, __uuidof(ID3D11DeviceContext));
_COM_SMARTPTR_TYPEDEF(IDXGIDevice, __uuidof(IDXGIDevice));
_COM_SMARTPTR_TYPEDEF(IDXGIOutput1, __uuidof(IDXGIOutput1));
_COM_SMARTPTR_TYPEDEF(IDXGIOutput, __uuidof(IDXGIOutput));
_COM_SMARTPTR_TYPEDEF(IDXGIAdapter1, __uuidof(IDXGIAdapter1));
_COM_SMARTPTR_TYPEDEF(IDXGIOutputDuplication, __uuidof(IDXGIOutputDuplication));
_COM_SMARTPTR_TYPEDEF(ID3D11Texture2D, __uuidof(ID3D11Texture2D));
_COM_SMARTPTR_TYPEDEF(IDXGISurface1, __uuidof(IDXGISurface1));
_COM_SMARTPTR_TYPEDEF(IDXGIResource, __uuidof(IDXGIResource));

_COM_SMARTPTR_TYPEDEF(ID3D11RenderTargetView, __uuidof(ID3D11RenderTargetView));
_COM_SMARTPTR_TYPEDEF(ID3D11ShaderResourceView, __uuidof(ID3D11ShaderResourceView));
_COM_SMARTPTR_TYPEDEF(ID3D11DepthStencilView, __uuidof(ID3D11DepthStencilView));
_COM_SMARTPTR_TYPEDEF(ID3D11InputLayout, __uuidof(ID3D11InputLayout));
_COM_SMARTPTR_TYPEDEF(ID3D11VertexShader, __uuidof(ID3D11VertexShader));
_COM_SMARTPTR_TYPEDEF(ID3D11PixelShader, __uuidof(ID3D11PixelShader));
_COM_SMARTPTR_TYPEDEF(ID3D11SamplerState, __uuidof(ID3D11SamplerState));
_COM_SMARTPTR_TYPEDEF(ID3D11Buffer, __uuidof(ID3D11Buffer));

_COM_SMARTPTR_TYPEDEF(IMFActivate, __uuidof(IMFActivate));
_COM_SMARTPTR_TYPEDEF(IMFAttributes, __uuidof(IMFAttributes));
_COM_SMARTPTR_TYPEDEF(IMFDXGIDeviceManager, __uuidof(IMFDXGIDeviceManager));
_COM_SMARTPTR_TYPEDEF(IMFTransform, __uuidof(IMFTransform));
_COM_SMARTPTR_TYPEDEF(IMFMediaEvent, __uuidof(IMFMediaEvent));
_COM_SMARTPTR_TYPEDEF(IMFMediaEventGenerator, __uuidof(IMFMediaEventGenerator));
_COM_SMARTPTR_TYPEDEF(IMFMediaType, __uuidof(IMFMediaType));
_COM_SMARTPTR_TYPEDEF(IMFSample, __uuidof(IMFSample));
_COM_SMARTPTR_TYPEDEF(IMFMediaBuffer, __uuidof(IMFMediaBuffer));
_COM_SMARTPTR_TYPEDEF(IMF2DBuffer, __uuidof(IMF2DBuffer));

//fnadales: Set New Frame timeout to 16~60FPS to avoid extra errors
#define ACQUIRENEXTFRAME_TIMEOUT 16

//fnadales: No output found error
#define NO_OUTPUTS_ERROR -666

using namespace std;

class Scaler;
class DXGIPointerInfo;

enum CaptureSource
{
	CSUndefined,
	CSMonitor1,
	CSMonitor2,
	CSDesktop
};

class DXGIPointerInfo
{
public:
	DXGIPointerInfo(BYTE* pPointerShape, UINT uiPointerShapeBufSize, DXGI_OUTDUPL_FRAME_INFO fi, DXGI_OUTDUPL_POINTER_SHAPE_INFO psi);
	~DXGIPointerInfo();
	BYTE* GetBuffer();
	UINT GetBufferSize();
	DXGI_OUTDUPL_FRAME_INFO& GetFrameInfo();
	DXGI_OUTDUPL_POINTER_SHAPE_INFO& GetShapeInfo();

private:
	BYTE* m_pPointerShape;
	UINT m_uiPointerShapeBufSize;
	DXGI_OUTDUPL_POINTER_SHAPE_INFO m_PSI;
	DXGI_OUTDUPL_FRAME_INFO m_FI;
};

class DXGIManager;
class DXGIOutputDuplication
{
	friend class DXGIManager;
public:

	DXGIOutputDuplication(IDXGIAdapter1* pAdapter,
		ID3D11Device* pD3DDevice,
		ID3D11DeviceContext* pD3DDeviceContext,
		IDXGIOutput1* pDXGIOutput1,
		IDXGIOutputDuplication* pDXGIOutputDuplication);
    ~DXGIOutputDuplication();
	
	HRESULT GetDesc(DXGI_OUTPUT_DESC& desc);
	HRESULT AcquireNextFrame(DXGIPointerInfo*& pDXGIPointer, ID3D11Texture2DPtr& outTexture);
	HRESULT ReleaseFrame();
	bool IsPrimary();

protected:
	CComPtr<ID3D11Device> m_D3DDevice;
	CComPtr<ID3D11DeviceContext> m_D3DDeviceContext;

private:
	CComPtr<IDXGIAdapter1> m_Adapter;
	
	CComPtr<IDXGIOutput1> m_DXGIOutput1;
	CComPtr<IDXGIOutputDuplication> m_DXGIOutputDuplication;

    CComPtr<IDXGIResource> m_CachedResource;
};

class DXGIManager
{
public:

	DXGIManager(unsigned int outWidth = 0, unsigned int outHeight = 0);
	~DXGIManager();
	HRESULT SetCaptureSource(CaptureSource type);
	CaptureSource GetCaptureSource();

    HRESULT GetOutputRect(RECT& rc); // Request scaled size
	HRESULT GetOriginalRect(RECT& rc); // Screen size

    HRESULT GetOutputBits(BYTE** pBits, DWORD& outLen, bool& nv12); // Buffer is created but owned by you.  NV12 format is used in HW.
    HRESULT IsSupported();

private:
	HRESULT Init();
	int GetMonitorCount();
	vector<DXGIOutputDuplication> GetOutputDuplication();
	void DrawMousePointer(BYTE* pDesktopBits, RECT rcDesktop, RECT rcDest);
	HRESULT ScaleFrame(ID3D11DevicePtr device, ID3D11DeviceContextPtr context, ID3D11Texture2DPtr inTexture, int width, int height, BYTE*& buffer, DWORD& outLen);

private:
	CComPtr<IDXGIFactory1> m_spDXGIFactory1;
	vector<DXGIOutputDuplication> m_vOutputs;
	bool m_bInitialized;
	CaptureSource m_CaptureSource;
	RECT m_rcCurrentOutput;
	BYTE* m_pBuf;

	CComPtr<IWICImagingFactory> m_spWICFactory;
	ULONG_PTR m_gdiplusToken;
	DXGIPointerInfo* m_pDXGIPointer;

	unsigned int _outWidth;
	unsigned int _outHeight;
	Scaler * _scaler;
	// Cache textures for scaling
	ID3D11Texture2DPtr _copyNV12Texture;
	ID3D11Texture2DPtr _copyRGBTexture;
    ID3D11Texture2DPtr _gdiTexture;
    POINT _lastCursor;
};

#include "DXGIManager.h"
#include <gdiplus.h>
#include <memory>
#include "Debug.h"
#include "pch.h"
using namespace Gdiplus;

DEFINE_GUID(MF_XVP_PLAYBACK_MODE, 0x3c5d293f, 0xad67, 0x4e29, 0xaf, 0x12, 0xcf, 0x3e, 0x23, 0x8a, 0xcc, 0xe9);

namespace DXUtils
{

    static ID3D11Texture2DPtr CreateTexture(ID3D11DevicePtr device, int width, int height, DXGI_FORMAT format)
    {
        static const int s_pixel = 0xffc99aff;

        D3D11_SUBRESOURCE_DATA initData = { &s_pixel, sizeof(int), 0 };

        D3D11_TEXTURE2D_DESC desc = {};
        desc.Width = width;
        desc.Height = height;
        desc.MipLevels = desc.ArraySize = 1;
        desc.Format = format;
        desc.SampleDesc.Count = 1;
        desc.Usage = D3D11_USAGE_DEFAULT;
        desc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;

        ID3D11Texture2D * texture;
        HRESULT hr = device->CreateTexture2D(&desc, &initData, &texture);
        if (FAILED(hr))
            return NULL;

        return texture;
    }

    static ID3D11Texture2DPtr CreateGDITexture(ID3D11DevicePtr device, int width, int height, DXGI_FORMAT format)
    {
        D3D11_TEXTURE2D_DESC desc = {};

        desc.Width = width;
        desc.Height = height;
        desc.Format = format;
        desc.MipLevels = desc.ArraySize = 1;
        desc.BindFlags = D3D11_BIND_RENDER_TARGET;
        desc.MiscFlags = D3D11_RESOURCE_MISC_GDI_COMPATIBLE;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.CPUAccessFlags = 0;
        desc.Usage = D3D11_USAGE_DEFAULT;

        ID3D11Texture2D * texture;
        HRESULT hr = device->CreateTexture2D(&desc, NULL, &texture);
        if (FAILED(hr))
            return NULL;

        return texture;
    }
}

class Scaler
{
public:

    Scaler()
    {
        _pVideoControl = nullptr;
        _initialised = false;
    }

    ~Scaler()
    {
        if (_pVideoProcessorMFT != nullptr)
            _pVideoProcessorMFT->Release();

        if (_pVideoControl != nullptr)
            _pVideoControl->Release();
    }

    bool Init(ID3D11DevicePtr device, int width, int height, int outWidth, int outHeight)
    {
        if (_initialised)
            return _initialised;

        _device = device;

        _height = height;
        _height = height;
        _width = width;
        _outWidth = (outWidth == 0) ? width : outWidth;
        _outHeight = (outHeight == 0) ? height : outHeight;

        HRESULT hr = CoCreateInstance(	CLSID_VideoProcessorMFT,
                                        nullptr,
                                        CLSCTX_INPROC_SERVER,
                                        IID_IMFTransform,
                                        (void**)&_pVideoProcessorMFT);

        CHECK_HR(hr);

        IMFAttributesPtr attribs;
        _pVideoProcessorMFT->GetAttributes(&attribs);
        attribs->SetUINT32(MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, TRUE);
        attribs->SetUINT32(MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, TRUE);
        attribs->SetUINT32(MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, TRUE);
        attribs->SetUINT32(MF_LOW_LATENCY, TRUE);

        UINT32 d3d11Aware = 0;
        UINT32 async = 0;
        attribs->GetUINT32(MF_TRANSFORM_ASYNC, &async);
        attribs->GetUINT32(MF_SA_D3D11_AWARE, &d3d11Aware);

        MFCreateDXGIDeviceManager(&_resetToken, &_deviceManager);
        hr = _deviceManager->ResetDevice(device, _resetToken);

        CHECK_HR(hr = _pVideoProcessorMFT->ProcessMessage(MFT_MESSAGE_SET_D3D_MANAGER, ULONG_PTR(_deviceManager.GetInterfacePtr())));

        // Tell the XVP that we are the swapchain allocator
        CHECK_HR(hr = _pVideoProcessorMFT->GetAttributes(&attribs));
        CHECK_HR(hr = attribs->SetUINT32(MF_XVP_PLAYBACK_MODE, TRUE));
        CHECK_HR(hr = _pVideoProcessorMFT->QueryInterface(IID_PPV_ARGS(&_pVideoControl)));

        // Output Type
        CHECK_HR(hr = MFCreateMediaType(&_outputType));

        CHECK_HR(hr = _outputType->SetGUID(MF_MT_MAJOR_TYPE, MFMediaType_Video));

        CHECK_HR(hr = _outputType->SetGUID(MF_MT_SUBTYPE, MFVideoFormat_NV12));

        CHECK_HR(hr = _outputType->SetUINT32(MF_MT_ALL_SAMPLES_INDEPENDENT, TRUE)); // UnCompressed

        CHECK_HR(hr = _outputType->SetUINT32(MF_MT_FIXED_SIZE_SAMPLES, TRUE)); // UnCompressed

        CHECK_HR(hr = _outputType->SetUINT32(MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive));

        CHECK_HR(hr = MFSetAttributeSize(_outputType, MF_MT_FRAME_SIZE, _outWidth, _outHeight));

        // Input Type
        CHECK_HR(hr = MFCreateMediaType(&_inputType));

        CHECK_HR(hr = _inputType->SetGUID(MF_MT_MAJOR_TYPE, MFMediaType_Video));

        CHECK_HR(hr = _inputType->SetGUID(MF_MT_SUBTYPE, MFVideoFormat_RGB32));

        CHECK_HR(hr = _inputType->SetUINT32(MF_MT_ALL_SAMPLES_INDEPENDENT, TRUE)); // UnCompressed

        CHECK_HR(hr = _inputType->SetUINT32(MF_MT_FIXED_SIZE_SAMPLES, TRUE)); // UnCompressed

        CHECK_HR(hr = _inputType->SetUINT32(MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive));

        CHECK_HR(hr = MFSetAttributeSize(_inputType, MF_MT_FRAME_SIZE, width, height));

        CHECK_HR(hr = _pVideoProcessorMFT->GetAttributes(&attribs));
        CHECK_HR(hr = attribs->SetUINT32(MF_XVP_PLAYBACK_MODE, TRUE));
        CHECK_HR(hr = _pVideoProcessorMFT->QueryInterface(IID_PPV_ARGS(&_pVideoControl)));

        // SET SIZES
        RECT dst = { 0, 0, (LONG)_outWidth, (LONG)_outHeight };
        CHECK_HR(hr = _pVideoControl->SetDestinationRectangle(&dst));

        RECT src = { 0,0, (LONG)_width, (LONG)_height };

        CHECK_HR(hr = _pVideoControl->SetSourceRectangle(&src));

        // Set input and output types on processor
        CHECK_HR(hr = _pVideoProcessorMFT->SetOutputType(0, _outputType, 0));
        CHECK_HR(hr = _pVideoProcessorMFT->SetInputType(0, _inputType, 0));

        hr = _pVideoProcessorMFT->ProcessMessage(MFT_MESSAGE_COMMAND_FLUSH, 0);
        hr = _pVideoProcessorMFT->ProcessMessage(MFT_MESSAGE_NOTIFY_BEGIN_STREAMING, 0);
        hr = _pVideoProcessorMFT->ProcessMessage(MFT_MESSAGE_NOTIFY_START_OF_STREAM, 0);

        _initialised = true;
        return _initialised;
    }

    ID3D11Texture2DPtr CreateTexture(int width, int height, DXGI_FORMAT format)
    {
        static const int s_pixel = 0xffc99aff;

        D3D11_SUBRESOURCE_DATA initData = { &s_pixel, sizeof(int), 0 };

        D3D11_TEXTURE2D_DESC desc = {};
        desc.Width = width;
        desc.Height = height;
        desc.MipLevels = desc.ArraySize = 1;
        desc.Format = format;
        desc.SampleDesc.Count = 1;
        desc.Usage = D3D11_USAGE_DEFAULT;
        desc.BindFlags = D3D11_BIND_SHADER_RESOURCE; //D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;

        ID3D11Texture2D * texture;
        HRESULT hr = _device->CreateTexture2D(&desc, &initData, &texture);
        if (FAILED(hr))
            return NULL;

        return texture;
    }

    bool ProcessInput(ID3D11Texture2DPtr texture, IMFSamplePtr& outSample)
    {
        // Create buffer from texture, we only do HW :-?
        HRESULT hr;
        IMFMediaBufferPtr inputBuffer;

        CHECK_HR(MFCreateDXGISurfaceBuffer(__uuidof(ID3D11Texture2D), texture, 0, FALSE, &inputBuffer));

        IMF2DBufferPtr imfInBuffer(inputBuffer);
        DWORD length;
        CHECK_HR(imfInBuffer->GetContiguousLength(&length));
        CHECK_HR(inputBuffer->SetCurrentLength(length));

        IMFSamplePtr sample;
        CHECK_HR(MFCreateSample(&sample));
        CHECK_HR(sample->AddBuffer(inputBuffer));
        CHECK_HR(sample->SetSampleTime(1));

        MFT_OUTPUT_STREAM_INFO mftStreamInfo = { 0 };
        MFT_OUTPUT_DATA_BUFFER mftOutputData = { 0 };

        CHECK_HR(hr = _pVideoProcessorMFT->GetOutputStreamInfo(0, &mftStreamInfo));
        if (_outputSample == NULL)
        {
            IMFMediaBufferPtr outBuffer;

            static const int s_pixel = 0xffc99aff;
            D3D11_SUBRESOURCE_DATA initData = { &s_pixel, sizeof(int), 0 };

            D3D11_TEXTURE2D_DESC desc = {};
            desc.Width = _outWidth;
            desc.Height = _outHeight;
            desc.MipLevels = desc.ArraySize = 1;
            desc.Format = DXGI_FORMAT_NV12;  //NV12
            desc.SampleDesc.Count = 1;
            desc.Usage = D3D11_USAGE_DEFAULT;
            desc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;

            CHECK_HR(hr = _device->CreateTexture2D(&desc, &initData, &_outTexture));
            CHECK_HR(hr = MFCreateDXGISurfaceBuffer(__uuidof(ID3D11Texture2D), _outTexture, 0, FALSE, &outBuffer));

            IMF2DBufferPtr imfOutBuffer(outBuffer);
            imfOutBuffer->GetContiguousLength(&length);
            outBuffer->SetCurrentLength(length);

            MFCreateSample(&_outputSample);
            _outputSample->AddBuffer(outBuffer);
        }
        // Set the output sample
        mftOutputData.pSample = _outputSample;
        //Set the output id
        mftOutputData.dwStreamID = 0;

        DWORD dwStatus = 0;
        // Check if we have anything in output
        hr = _pVideoProcessorMFT->ProcessOutput(0, 1, &mftOutputData, &dwStatus);

        if (hr == MF_E_TRANSFORM_NEED_MORE_INPUT)
        {
            // Send input sample to be processed
            CHECK_HR(hr = _pVideoProcessorMFT->ProcessInput(0, sample, 0));
            CHECK_HR(hr = _pVideoProcessorMFT->ProcessOutput(0, 1, &mftOutputData, &dwStatus));

            outSample = mftOutputData.pSample;
        }
        else if (FAILED(hr))
        {
            printf("SCALER: Failed to process output: %08x", (int)hr);
        }
        return true;
    }

    unsigned int outWidth()
    {
        return _outWidth;
    }

    unsigned int outHeight()
    {
        return _outHeight;
    }

    unsigned int width()
    {
        return _width;
    }

    unsigned int height()
    {
        return _height;
    }

private:
    unsigned int _width;
    unsigned int _height;
    unsigned int _outWidth;
    unsigned int _outHeight;
    IMFTransformPtr _pVideoProcessorMFT;
    IMFMediaTypePtr _inputType;
    IMFMediaTypePtr _outputType;
    ID3D11DevicePtr _device;
    UINT _resetToken;
    IMFDXGIDeviceManagerPtr _deviceManager;
    ID3D11Texture2DPtr _outTexture;
    IMFSamplePtr _outputSample;
    IMFVideoProcessorControl* _pVideoControl;
    bool _initialised;

};

DXGIPointerInfo::DXGIPointerInfo(BYTE* pPointerShape, UINT uiPointerShapeBufSize, DXGI_OUTDUPL_FRAME_INFO fi, DXGI_OUTDUPL_POINTER_SHAPE_INFO psi)
    : m_pPointerShape(pPointerShape)
    , m_uiPointerShapeBufSize(uiPointerShapeBufSize)
    , m_FI(fi)
    , m_PSI(psi)
{
}

DXGIPointerInfo::~DXGIPointerInfo()
{
    if(m_pPointerShape)
    {
        delete [] m_pPointerShape;
    }
}

BYTE* DXGIPointerInfo::GetBuffer()
{
    return m_pPointerShape;
}

UINT DXGIPointerInfo::GetBufferSize()
{
    return m_uiPointerShapeBufSize;
}

DXGI_OUTDUPL_FRAME_INFO& DXGIPointerInfo::GetFrameInfo()
{
    return m_FI;
}

DXGI_OUTDUPL_POINTER_SHAPE_INFO& DXGIPointerInfo::GetShapeInfo()
{
    return m_PSI;
}

DXGIOutputDuplication::DXGIOutputDuplication(IDXGIAdapter1* pAdapter,
                                             ID3D11Device* pD3DDevice,
                                             ID3D11DeviceContext* pD3DDeviceContext,
                                             IDXGIOutput1* pDXGIOutput1,
                                             IDXGIOutputDuplication* pDXGIOutputDuplication)
    : m_Adapter(pAdapter)
    , m_D3DDevice(pD3DDevice)
    , m_D3DDeviceContext(pD3DDeviceContext)
    , m_DXGIOutput1(pDXGIOutput1)
    , m_DXGIOutputDuplication(pDXGIOutputDuplication)
{
}

DXGIOutputDuplication::~DXGIOutputDuplication()
{
    m_Adapter.Release();
    m_D3DDevice.Release();
    m_D3DDeviceContext.Release();
    m_DXGIOutput1.Release();
    m_DXGIOutputDuplication.Release();
}

HRESULT DXGIOutputDuplication::GetDesc(DXGI_OUTPUT_DESC& desc)
{
    m_DXGIOutput1->GetDesc(&desc);
    return S_OK;
}

//#define WRITE_FILE
#ifdef WRITE_FILE
FILE * f1 = fopen("C:\\Users\\andyb\\dxgimanager1.nv12", "wb");
FILE * f2 = fopen("C:\\Users\\andyb\\dxgimanager2.nv12", "wb");
FILE * f3 = fopen("C:\\Users\\andyb\\dxgimanager3.nv12", "wb");
FILE * f4 = fopen("C:\\Users\\andyb\\dxgimanager4.nv12", "wb");

HRESULT DownloadTexture(FILE * f, ID3D11Texture2DPtr copyTexture, ID3D11DevicePtr device, ID3D11DeviceContextPtr context)
{
    D3D11_TEXTURE2D_DESC desc;
    copyTexture->GetDesc(&desc);

    D3D11_TEXTURE2D_DESC texDesc;
    ZeroMemory(&texDesc, sizeof(texDesc));
    texDesc.Width = desc.Width;
    texDesc.Height = desc.Height;
    texDesc.MipLevels = 1;
    texDesc.ArraySize = 1;
    texDesc.SampleDesc.Count = 1;
    texDesc.SampleDesc.Quality = 0;
    texDesc.Usage = D3D11_USAGE_STAGING;
    texDesc.Format = desc.Format;
    texDesc.BindFlags = 0;
    texDesc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texDesc.MiscFlags = 0;

    CComPtr<ID3D11Texture2D> spD3D11Texture2D = NULL;
    HRESULT hr = device->CreateTexture2D(&texDesc, NULL, &spD3D11Texture2D);
    if (FAILED(hr))
        return hr;

    context->CopyResource(spD3D11Texture2D, copyTexture);
    D3D11_MAPPED_SUBRESOURCE mapping;
    hr = context->Map(spD3D11Texture2D, 0, D3D11_MAP_READ, 0, &mapping);

    const unsigned int size = desc.Width * desc.Height * 4;
    unsigned char * buffer = new unsigned char[size];
    memcpy(buffer, mapping.pData, size);

    context->Unmap(spD3D11Texture2D, 0);

    fwrite(buffer, 1, size, f);
    delete buffer;
}
#endif


HRESULT DXGIOutputDuplication::AcquireNextFrame(DXGIPointerInfo*& pDXGIPointer, ID3D11Texture2DPtr & outTexture)
{
    DXGI_OUTDUPL_FRAME_INFO fi;
    CComPtr<IDXGIResource> spDXGIResource;

    HRESULT hr = m_DXGIOutputDuplication->AcquireNextFrame(ACQUIRENEXTFRAME_TIMEOUT, &fi, &spDXGIResource);
    if(FAILED(hr))
    {
        if ((hr != DXGI_ERROR_ACCESS_LOST) && (hr != DXGI_ERROR_WAIT_TIMEOUT)) {
            DEBUG_INFO("m_DXGIOutputDuplication->AcquireNextFrame failed with hr=0x%08x", hr);
        }
        return hr;
    }

    hr = spDXGIResource->QueryInterface(IID_PPV_ARGS(&outTexture));

    if (FAILED(hr))
    {
        return hr;
    }
#ifdef WRITE_FILE
    DownloadTexture(f1, outTexture, m_D3DDevice.p, m_D3DDeviceContext.p);
#endif

    // Updating mouse pointer, if visible
    if(fi.PointerPosition.Visible)
    {
        BYTE* pPointerShape = new BYTE[fi.PointerShapeBufferSize];

        DXGI_OUTDUPL_POINTER_SHAPE_INFO psi = {};
        UINT uiPointerShapeBufSize = fi.PointerShapeBufferSize;
        hr = m_DXGIOutputDuplication->GetFramePointerShape(uiPointerShapeBufSize, pPointerShape, &uiPointerShapeBufSize, &psi);
        if(hr == DXGI_ERROR_MORE_DATA)
        {
            pPointerShape = new BYTE[uiPointerShapeBufSize];

            hr = m_DXGIOutputDuplication->GetFramePointerShape(uiPointerShapeBufSize, pPointerShape, &uiPointerShapeBufSize, &psi);
        }

        if(hr == S_OK)
        {
//            qInfo("PointerPosition Visible=%d x=%d y=%d w=%d h=%d type=%d\n", fi.PointerPosition.Visible, fi.PointerPosition.Position.x, fi.PointerPosition.Position.y, psi.Width, psi.Height, psi.Type);

            if((psi.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME ||
                psi.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR ||
                psi.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR) &&
                    psi.Width <= 128 && psi.Height <= 128)
            {
                // Here we can obtain pointer shape
                if(pDXGIPointer)
                {
                    delete pDXGIPointer;
                }

                pDXGIPointer = new DXGIPointerInfo(pPointerShape, uiPointerShapeBufSize, fi, psi);

                pPointerShape = NULL;
            }

            DXGI_OUTPUT_DESC outDesc;
            GetDesc(outDesc);

            if(pDXGIPointer)
            {
                pDXGIPointer->GetFrameInfo().PointerPosition.Position.x = outDesc.DesktopCoordinates.left + fi.PointerPosition.Position.x;
                pDXGIPointer->GetFrameInfo().PointerPosition.Position.y = outDesc.DesktopCoordinates.top + fi.PointerPosition.Position.y;
            }
        }

        if(pPointerShape)
        {
            delete [] pPointerShape;
        }
    }

    return hr;
}

HRESULT DXGIOutputDuplication::ReleaseFrame()
{
    m_DXGIOutputDuplication->ReleaseFrame();
    return S_OK;
}

bool DXGIOutputDuplication::IsPrimary()
{
    DXGI_OUTPUT_DESC outdesc;
    m_DXGIOutput1->GetDesc(&outdesc);

    MONITORINFO mi;
    mi.cbSize = sizeof(MONITORINFO);
    GetMonitorInfo(outdesc.Monitor, &mi);
    if(mi.dwFlags & MONITORINFOF_PRIMARY)
    {
        return true;
    }
    return false;
}

DXGIManager::DXGIManager(unsigned int outWidth, unsigned int outHeight)
{
    m_CaptureSource = CSUndefined;
    SetRect(&m_rcCurrentOutput, 0, 0, 0, 0);
    m_pBuf = NULL;
    m_pDXGIPointer = NULL;
    m_bInitialized = false;
    m_gdiplusToken = 0;
    _outHeight = outHeight;
    _outWidth = outWidth;
    _scaler = new Scaler;
    _copyNV12Texture = _copyRGBTexture = _gdiTexture = nullptr;
}

DXGIManager::~DXGIManager()
{
    if(m_pBuf != NULL)
        delete [] m_pBuf;
    m_pBuf = NULL;

    if (_scaler != NULL)
        delete _scaler;

    if(m_pDXGIPointer != NULL)
        delete m_pDXGIPointer;

    m_pDXGIPointer = NULL;

    m_vOutputs.clear();

    m_spDXGIFactory1.Release();
    m_spWICFactory.Release();

    GdiplusShutdown(m_gdiplusToken);
}

HRESULT DXGIManager::SetCaptureSource(CaptureSource cs)
{
    m_CaptureSource = cs;
    return S_OK;
}

CaptureSource DXGIManager::GetCaptureSource()
{
    return m_CaptureSource;
}

HRESULT DXGIManager::Init()
{
    if(m_bInitialized)
        return S_OK;

    GdiplusStartupInput gdiplusStartupInput;
    GdiplusStartup(&m_gdiplusToken, &gdiplusStartupInput, NULL);

    HRESULT hr = CreateDXGIFactory1(__uuidof(IDXGIFactory1), (void**)(&m_spDXGIFactory1) );
    if( FAILED(hr) )
    {
        DEBUG_WARN("Failed to CreateDXGIFactory1 hr=%08x", hr);
        return hr;
    }

    // Getting all adapters
    vector<CComPtr<IDXGIAdapter1>> vAdapters;

    CComPtr<IDXGIAdapter1> spAdapter;
    for(int i=0; m_spDXGIFactory1->EnumAdapters1(i, &spAdapter) != DXGI_ERROR_NOT_FOUND; i++)
    {
        vAdapters.push_back(spAdapter);
        spAdapter.Release();
    }

    // Iterating over all adapters to get all outputs
    for(vector<CComPtr<IDXGIAdapter1>>::iterator AdapterIter = vAdapters.begin();
        AdapterIter != vAdapters.end();
        AdapterIter++)
    {
        vector<CComPtr<IDXGIOutput>> vOutputs;

        CComPtr<IDXGIOutput> spDXGIOutput;
        for(int i=0; (*AdapterIter)->EnumOutputs(i, &spDXGIOutput) != DXGI_ERROR_NOT_FOUND; i++)
        {
            DXGI_OUTPUT_DESC outputDesc;
            spDXGIOutput->GetDesc(&outputDesc);

            DEBUG_INFO("Display output found. DeviceName=%ls  AttachedToDesktop=%d Rotation=%d DesktopCoordinates={(%d,%d),(%d,%d)}",
                  outputDesc.DeviceName,
                  outputDesc.AttachedToDesktop,
                  outputDesc.Rotation,
                  outputDesc.DesktopCoordinates.left,
                  outputDesc.DesktopCoordinates.top,
                  outputDesc.DesktopCoordinates.right,
                  outputDesc.DesktopCoordinates.bottom);

            if(outputDesc.AttachedToDesktop)
            {
                vOutputs.push_back(spDXGIOutput);
            }

            spDXGIOutput.Release();
        }

        if(vOutputs.size() == 0)
            continue;

        // Creating device for each adapter that has the output
        CComPtr<ID3D11Device> spD3D11Device;
        CComPtr<ID3D11DeviceContext> spD3D11DeviceContext;

        static const D3D_FEATURE_LEVEL featureLevels[] = {
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1
        };

#if defined (DEBUG) || defined(QT_DEBUG)
//Have in account that there are devices that does not allow to create device with debug flag
#define DEVICE_CREATE_FLAGS (D3D11_CREATE_DEVICE_DEBUG)
#else
#define DEVICE_CREATE_FLAGS (0)
#endif

        hr = D3D11CreateDevice(
            (*AdapterIter),
            D3D_DRIVER_TYPE_UNKNOWN,
            NULL,
            DEVICE_CREATE_FLAGS | D3D11_CREATE_DEVICE_VIDEO_SUPPORT,
            featureLevels, ARRAYSIZE(featureLevels),
            D3D11_SDK_VERSION,
            &spD3D11Device,
            NULL,
            &spD3D11DeviceContext
        );

        if ( FAILED(hr) ) {
            DEBUG_WARN("Can't use D3D11_CREATE_DEVICE_VIDEO_SUPPORT, maybe there's not WDDM; trying without it");
            hr = D3D11CreateDevice(
                (*AdapterIter),
                D3D_DRIVER_TYPE_UNKNOWN,
                NULL,
                // if WDDM is not implemented it's going to fail with D3D11_CREATE_DEVICE_VIDEO_SUPPORT according to doc
                // https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/ne-d3d11-d3d11_create_device_flag
                DEVICE_CREATE_FLAGS,
                featureLevels, ARRAYSIZE(featureLevels),
                D3D11_SDK_VERSION,
                &spD3D11Device,
                NULL,
                &spD3D11DeviceContext
            );
        }

        if( FAILED(hr) )
        {
            DEBUG_WARN("Failed to create D3D11CreateDevice hr=%08x", hr);
            return hr;
        }

        // Setting us as high priority.
        CComPtr<IDXGIDevice> dxgi;
        hr = spD3D11Device->QueryInterface(__uuidof(IDXGIDevice), (void **)&dxgi);
        if (FAILED(hr)){
            DEBUG_WARN("Failed to get IDXGIDevice interface hr=%08x", hr);
        }else{
            /*
             * Positive values increase the likelihood that the GPU scheduler will grant GPU execution cycles to the device when rendering.
             * https://msdn.microsoft.com/en-us/library/windows/desktop/bb174534(v=vs.85).aspx
             */
            dxgi->SetGPUThreadPriority(7);
        }

        for(std::vector<CComPtr<IDXGIOutput>>::iterator OutputIter = vOutputs.begin();
            OutputIter != vOutputs.end();
            OutputIter++)
        {
            CComQIPtr<IDXGIOutput1> spDXGIOutput1 = *OutputIter;
            CComQIPtr<IDXGIDevice1> spDXGIDevice = spD3D11Device;
            if(!spDXGIOutput1 || !spDXGIDevice)
                continue;

            CComPtr<IDXGIOutputDuplication> spDXGIOutputDuplication;
            hr = spDXGIOutput1->DuplicateOutput(spDXGIDevice, &spDXGIOutputDuplication);
            if( FAILED(hr) )
            {
                DEBUG_WARN("Failed DuplicateOutput hr=%08x", hr);
                continue;
            }

            m_vOutputs.push_back(
                        DXGIOutputDuplication((*AdapterIter),
                                              spD3D11Device,
                                              spD3D11DeviceContext,
                                              spDXGIOutput1,
                                              spDXGIOutputDuplication));
        }
        if (m_vOutputs.size() == 0)
        {
            DEBUG_WARN("No vOutputs available!");
            return -1;
        }
    }

    hr = m_spWICFactory.CoCreateInstance(CLSID_WICImagingFactory);
    if( FAILED(hr) )
    {
        DEBUG_WARN("Failed to create WICImagingFactory hr=%08x", hr);
        return hr;
    }

    m_bInitialized = true;

    return S_OK;
}

HRESULT DXGIManager::GetOutputRect(RECT& rc)
{
    // Nulling rc just in case...
    SetRect(&rc, 0, 0, 0, 0);

    HRESULT hr = Init();
    if(hr != S_OK)
        return hr;

    vector<DXGIOutputDuplication> vOutputs = GetOutputDuplication();

    RECT rcShare;
    SetRect(&rcShare, 0, 0, 0, 0);

    for(vector<DXGIOutputDuplication>::iterator iter = vOutputs.begin();
        iter != vOutputs.end();
        iter++)
    {
        DXGIOutputDuplication& out = *iter;

        DXGI_OUTPUT_DESC outDesc;
        out.GetDesc(outDesc);
        RECT rcOutCoords = outDesc.DesktopCoordinates;

        UnionRect(&rcShare, &rcShare, &rcOutCoords);
    }

    RECT scaledRect = { 0, 0, _outWidth, _outHeight };

    // Check scaling, if we have a scaled rectangle check that it is smaller than the desktop, we don't upscale
    if (scaledRect.right > 0 && scaledRect.bottom > 0 &&
        scaledRect.right < rcShare.right &&
        scaledRect.bottom < rcShare.bottom)
    {
        CopyRect(&rc, &scaledRect);
    }
    else
    {
        CopyRect(&rc, &rcShare);
    }

    return S_OK;
}

HRESULT DXGIManager::GetOriginalRect(RECT & rc)
{
    // Nulling rc just in case...
    SetRect(&rc, 0, 0, 0, 0);

    HRESULT hr = Init();
    if (hr != S_OK) {
        DEBUG_WARN("Couldn't initilize DXGI");
        return hr;
    }

    vector<DXGIOutputDuplication> vOutputs = GetOutputDuplication();

    RECT rcShare;
    SetRect(&rcShare, 0, 0, 0, 0);

    for (vector<DXGIOutputDuplication>::iterator iter = vOutputs.begin();
        iter != vOutputs.end();
        iter++)
    {
        DXGIOutputDuplication& out = *iter;

        DXGI_OUTPUT_DESC outDesc;
        out.GetDesc(outDesc);
        RECT rcOutCoords = outDesc.DesktopCoordinates;

        UnionRect(&rcShare, &rcShare, &rcOutCoords);
    }

    CopyRect(&rc, &rcShare);

    return hr;
}

HRESULT DXGIManager::GetOutputBits(BYTE** pBits, DWORD& outLen, bool& nv12)
{
    HRESULT hr = S_OK;

    RECT rcDest;
    hr = GetOutputRect(rcDest);

    if (FAILED(hr))
    {
        return hr;
    }

    RECT rcOutput;
    hr = GetOutputRect(rcOutput);
    if (FAILED(hr))
    {
        return hr;
    }

    DWORD dwOutputWidth = rcOutput.right - rcOutput.left;
    DWORD dwOutputHeight = rcOutput.bottom - rcOutput.top;

    BYTE* pBuf = NULL;
    if(outLen>0)
        pBuf = *pBits;

    vector<DXGIOutputDuplication> vOutputs = GetOutputDuplication();
	if (vOutputs.size() == 0)
		return NO_OUTPUTS_ERROR;

    for(vector<DXGIOutputDuplication>::iterator iter = vOutputs.begin();
        iter != vOutputs.end();
        iter++)
    {
        DXGIOutputDuplication& out = *iter;

        DXGI_OUTPUT_DESC outDesc;
        out.GetDesc(outDesc);
        RECT rcOutCoords = outDesc.DesktopCoordinates;

        ID3D11Texture2DPtr texture;
        // We get the texture from the grab, this will be a RGB32 texture from grabber
        hr = out.AcquireNextFrame(m_pDXGIPointer, texture);

        if (FAILED(hr))
        {
			return hr;
        }

        D3D11_TEXTURE2D_DESC desc;
        texture->GetDesc(&desc);

        if (_gdiTexture == nullptr)
        {
            _gdiTexture = DXUtils::CreateGDITexture(out.m_D3DDevice.p, desc.Width, desc.Height, desc.Format);
        }

        if (_gdiTexture != nullptr)
        {
             out.m_D3DDeviceContext.p->CopyResource(_gdiTexture, texture);
             IDXGISurface1Ptr surface;

             hr = _gdiTexture->QueryInterface(IID_PPV_ARGS(&surface));

             if (FAILED(hr))
             {
                 DEBUG_PRINT("Failed to query surface from GDI texture");
                 break;
             }

             CURSORINFO lCursorInfo = { 0 };

             lCursorInfo.cbSize = sizeof(lCursorInfo);

             auto lBoolres = GetCursorInfo(&lCursorInfo);

             if (lBoolres == TRUE)
             {
                 if (lCursorInfo.flags == CURSOR_SHOWING)
                 {
                     auto lCursorPosition = lCursorInfo.ptScreenPos;

                     auto lCursorSize = lCursorInfo.cbSize;

                     HDC  lHDC;

                     surface->GetDC(FALSE, &lHDC);

                     DrawIconEx(
                         lHDC,
                         lCursorPosition.x,
                         lCursorPosition.y,
                         lCursorInfo.hCursor,
                         0,
                         0,
                         0,
                         0,
                         DI_NORMAL | DI_DEFAULTSIZE);

                     surface->ReleaseDC(nullptr);
                 }
             }
        }
        else
        {
            DEBUG_WARN("Cursor not included in grab");
        }
#ifdef WRITE_FILE
        DownloadTexture(f2, texture, out.m_D3DDevice.p, out.m_D3DDeviceContext.p);
#endif
        // We have the texture from the grabber, now pass through scaler and convert to NV12 and resize if needed and download to memory
        hr = ScaleFrame(out.m_D3DDevice.p,
                        out.m_D3DDeviceContext.p,
                        (_gdiTexture != nullptr) ? _gdiTexture : texture,
                        dwOutputWidth,
                        dwOutputHeight,
                        pBuf,
                        outLen);

        if (FAILED(hr))
        {
            DEBUG_WARN("Scale failed");

            hr = S_OK;
            nv12 = false;
            // Download frame from texture as RGB32
            D3D11_TEXTURE2D_DESC desc;
            texture->GetDesc(&desc);

            if (_copyRGBTexture == nullptr)
            {
                D3D11_TEXTURE2D_DESC texDesc;
                ZeroMemory(&texDesc, sizeof(texDesc));
                texDesc.Width = desc.Width;
                texDesc.Height = desc.Height;
                texDesc.MipLevels = 1;
                texDesc.ArraySize = 1;
                texDesc.SampleDesc.Count = 1;
                texDesc.SampleDesc.Quality = 0;
                texDesc.Usage = D3D11_USAGE_STAGING;
                texDesc.Format = desc.Format;
                texDesc.BindFlags = 0;
                texDesc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
                texDesc.MiscFlags = 0;

                hr = out.m_D3DDevice.p->CreateTexture2D(&texDesc, NULL, &_copyRGBTexture);
            }

            if (FAILED(hr))
            {
                return hr;
            }
            out.m_D3DDeviceContext.p->CopyResource(_copyRGBTexture, (_gdiTexture != nullptr) ? _gdiTexture : texture);
            D3D11_MAPPED_SUBRESOURCE mapping;
            hr = out.m_D3DDeviceContext.p->Map(_copyRGBTexture, 0, D3D11_MAP_READ, 0, &mapping);

            if (FAILED(hr))
            {
                return hr;
            }

            const unsigned int size = desc.Width * desc.Height * 4;
            if(pBuf==NULL)
                pBuf = new BYTE[size];
            outLen = size;
            memcpy(pBuf, mapping.pData, size);

            out.m_D3DDeviceContext.p->Unmap(_copyRGBTexture, 0);
        }
        else
        {
            nv12 = true;
        }

        *pBits = pBuf;

        out.ReleaseFrame();
    }

    return hr;
}

HRESULT DXGIManager::ScaleFrame(ID3D11DevicePtr device, ID3D11DeviceContextPtr context, ID3D11Texture2DPtr inTexture, int width, int height, BYTE*& buffer, DWORD& outLen)
{
    HRESULT hr = S_OK;

    if (_scaler != nullptr)
    {
        D3D11_TEXTURE2D_DESC desc;
        inTexture->GetDesc(&desc);

        if (!_scaler->Init(device, desc.Width, desc.Height, width, height))
        {
            hr = -1;
        }
        else
        {

#ifdef WRITE_FILE
            DownloadTexture(f3, inTexture, device, context);
#endif
            // Copy inTexture in GPU to access
            // The grabbed texture should be DXGI_FORMAT_B8G8R8A8_UNORM
            if (_copyNV12Texture == nullptr)
                _copyNV12Texture = DXUtils::CreateTexture(device, desc.Width, desc.Height, desc.Format);

            D3D11_TEXTURE2D_DESC inDesc;
            inTexture->GetDesc(&inDesc);

            if (_copyNV12Texture != nullptr)
            {
                context->CopyResource(_copyNV12Texture, inTexture);
#ifdef WRITE_FILE
                DownloadTexture(f4, copyTexture, device, context);
#endif
                IMFSamplePtr outSample;

                if (!_scaler->ProcessInput(_copyNV12Texture, outSample))
                {
                    hr = -1;
                }
                else
                {
                    IMFMediaBufferPtr mb;
                    outSample->ConvertToContiguousBuffer(&mb);

                    BYTE *pixels;
                    mb->Lock(&pixels, NULL, &outLen);
                    if(buffer==NULL)
                        buffer = new BYTE[outLen];
                    memcpy(buffer, pixels, outLen);
                    mb->Unlock();
                }
            }
            else
            {

                hr = -1;
            }
        }
    }

    if (FAILED(hr))
    {
        if (_copyNV12Texture != nullptr)
        {
            _copyNV12Texture->Release();
            _copyNV12Texture = nullptr;
        }
    }

    return hr;
}

HRESULT DXGIManager::IsSupported()
{
    return Init();
}

vector<DXGIOutputDuplication> DXGIManager::GetOutputDuplication()
{
    vector<DXGIOutputDuplication> outputs;
    switch(m_CaptureSource)
    {
    case CSMonitor1:
    {
        // Return the one with IsPrimary
        for(vector<DXGIOutputDuplication>::iterator iter = m_vOutputs.begin();
            iter != m_vOutputs.end();
            iter++)
        {
            DXGIOutputDuplication& out = *iter;
            if(out.IsPrimary())
            {
                outputs.push_back(out);
                break;
            }
        }
    }
        break;

    case CSMonitor2:
    {
        // Return the first with !IsPrimary
        for(vector<DXGIOutputDuplication>::iterator iter = m_vOutputs.begin();
            iter != m_vOutputs.end();
            iter++)
        {
            DXGIOutputDuplication& out = *iter;
            if(!out.IsPrimary())
            {
                outputs.push_back(out);
                break;
            }
        }
    }
        break;

    case CSDesktop:
    {
        // Return all outputs
        for(vector<DXGIOutputDuplication>::iterator iter = m_vOutputs.begin();
            iter != m_vOutputs.end();
            iter++)
        {
            DXGIOutputDuplication& out = *iter;
            outputs.push_back(out);
        }
    }
        break;
    }
    return outputs;
}

BOOL CALLBACK MonitorEnumProc(HMONITOR hMonitor, HDC hdcMonitor, LPRECT lprcMonitor, LPARAM dwData)
{
    int *Count = (int*)dwData;
    (*Count)++;
    return TRUE;
}

int DXGIManager::GetMonitorCount()
{
    int Count = 0;
    if (EnumDisplayMonitors(NULL, NULL, MonitorEnumProc, (LPARAM)&Count))
        return Count;
    return -1;
}

#include "encoder.h"
#include "app.h"
#include <QMutexLocker>

extern "C" {
#include <libavcodec/avcodec.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libswscale/swscale.h>
}

namespace Supernova {

    static int NumberOfThreads(int width, int height, int number_of_cores) 
	{
		if (width * height >= 1920 * 1080 && number_of_cores > 8)
		{
			return 8;  // 8 threads for 1080p on high perf machines.
		} 
		else if (width * height > 1280 * 960 && number_of_cores >= 6) 
		{
			return 3;  // 3 threads for 1080p.
		} 
		else if (width * height > 640 * 480 && number_of_cores >= 3) 
		{
			return 2;  // 2 threads for qHD/HD.
		} 
		else 
		{
			return 1;  // 1 thread for VGA or less.
		}
    }

    Encoder::Encoder(QObject *parent) : QObject(parent)
    {
        _codec = nullptr;
        _codecContext = nullptr;
        _swsContext = nullptr;
        _initialised = _configured = false;
        _hasLocalRenderer = false;
        _capturedFrame = nullptr;
        _lastTime = 0;
    }

    Encoder::~Encoder()
    {
        deinit();
    }

    bool Encoder::init(const QSize & resolution, qint32 bitrate, qint32 framerate, EncodedFrameHeader::Codec codec)
    {
		if (_initialised)
			return _initialised;

        _frameCodec = codec;

		if (!initEncoder(resolution, bitrate, framerate))
			return false;

        connect(&_captureStatsTimer, &QTimer::timeout, [=] {
            qint64 now = QDateTime::currentMSecsSinceEpoch();
            _lastTime = now;
            _captures = 0;
        });

		_captureStatsTimer.start(1000);

        // At the moment just return true but we need to initialise the encoder implementation still
        return true;
    }

    void Encoder::deinit()
    {
        _timer.stop();
        if (_capturedFrame)
            delete _capturedFrame;
        _capturedFrame = nullptr;

        if (!_initialised) return;

        deinitEncoder();

        _initialised = false;
    }

    void Encoder::capturedFrame(CapturedFrame * frame)
    {
        _captures++;

        _timer.stop();

        if (!_initialised) {
            delete frame;
			return;
        }
		
        delete _capturedFrame;
        _capturedFrame = frame;
    }

    void Encoder::encodeFrame()
    {
		if (_capturedFrame == nullptr)
        {
            _timer.singleShot(1, this, SLOT(encodeFrame()));
            return;
        }

        // Take the last captured frame
        CapturedFrame * frame = _capturedFrame;
		// Encode frame
        if ( frame->dataSize > 0 && _initialised )
        {
			if (!initCodecContext(frame->format))
			{
				qWarning() << "Failed to initialise codec, not much more we can do after this.";
				return;
			}
			
			AVFrame * inFrame = av_frame_alloc();
			inFrame->format = _codecContext->pix_fmt;
			
			// Default, we clear the frame we are about to create
			bool clearFrame = true;
			int r = 0;
			
			if (frame->format == AV_PIX_FMT_NV12)
			{
				clearFrame = false;  // We don't own this frame, we are wrapping it and passing to encoder	
				// Already scaled, so use resolution.
				inFrame->width = frame->resolution.width();
				inFrame->height = frame->resolution.height();

				avpicture_fill((AVPicture*)inFrame, frame->data, _codecContext->pix_fmt, inFrame->width, inFrame->height);
			}
			else if(frame->format == AV_PIX_FMT_BGRA)// This will be a RGB32 frame which we need to scale to YUV420p
			{
				// Not scaled, we need to do that here.  Create out yuv420p frame at the size we want it
				inFrame->width = frame->resolution.width();
				inFrame->height = frame->resolution.height();
				
				if (inFrame->width > MAX_FRAME_WIDTH || inFrame->height > MAX_FRAME_HEIGHT)
				{
					qWarning() << "Invalid frame size";
					return;
				}

				r = av_image_alloc(inFrame->data, inFrame->linesize, inFrame->width, inFrame->height, _codecContext->pix_fmt, 32);
				
				static int pts = 0;
				inFrame->pts = pts++;

				_swsContext = sws_getCachedContext(_swsContext,
					frame->screenResolution.width(),
					frame->screenResolution.height(),
					frame->format /* AV_PIX_FMT_BGRA */,
					_codecContext->width,
					_codecContext->height,
					static_cast<AVPixelFormat>(inFrame->format) /* AV_PIX_FMT_YUV420P */,
					0, 0, 0, 0);

				const int inLinesize[1] = { 4 * frame->screenResolution.width() };
				uint8_t *rgb = (uint8_t*)frame->data;

				// TODO: despite having width resolutions that are multiple of 16, for some resolutions
				// (specially small ones) this warning is shown after calling sws_scale:
				//
				// "Warning: data is not aligned! This can lead to a speedloss"
				sws_scale(	_swsContext,
							const_cast<const uint8_t * const *>(&rgb), inLinesize,
							0, frame->screenResolution.height(),
							inFrame->data, inFrame->linesize);
			}
			else
			{
				qWarning() << "Unrecognised format, failed to encode.";
				avpicture_free((AVPicture*)inFrame);
				return;
			}
            
            AVPacket pkt;
            av_init_packet(&pkt);
            pkt.data = NULL;    // packet data will be allocated by the encoder
            pkt.size = 0;
            int got_output = 0;
            r = avcodec_encode_video2(_codecContext, &pkt, inFrame, &got_output);

    #ifdef WRITE_TO_FILE
            static FILE * file = fopen("C:\\Users\\andyb\\video_out.h264", "wb");
    #endif
            if (r >= 0 && got_output)
            {
                EncodedFrame * encodedFrame = new EncodedFrame(pkt.data, pkt.size);
    #ifdef WRITE_TO_FILE
                fwrite(pkt.data, 1, pkt.size, file);
    #endif
                encodedFrame->header.width = static_cast<quint16>(frame->resolution.width());
                encodedFrame->header.height = static_cast<quint16>(frame->resolution.height());
                encodedFrame->header.captureTime = frame->time;
                encodedFrame->header.encodedTime = getMillis();
                encodedFrame->header.codec = EncodedFrameHeader::Codec::H264_X264;

                av_packet_unref(&pkt);
                Q_EMIT frameEncoded(encodedFrame);
            }
            else
            {
                qWarning() << "Encode of packet failed";
            }

			if (clearFrame)
				avpicture_free( (AVPicture*)inFrame );
        }
        else
        {
			qWarning() << "Not initialised or frame is empty";
        }

		if (_hasLocalRenderer)
            Q_EMIT newLocalFrame(frame);
    }

    bool Encoder::setFrameRate(qint32 rate)
    {
        if (!_initialised || _codec == nullptr || _codecContext == nullptr)
            return false;

        avcodec_close(_codecContext);
        _rate = rate;

		_configured = false;

        return true;
    }

    bool Encoder::setBitRate(qint32 bitrate)
    {
        if (!_initialised || _codec == nullptr || _codecContext == nullptr)
            return false;

        avcodec_close(_codecContext);
        _bitrate = bitrate;

        _configured = false;
        
		return true;
    }

    void Encoder::onHasLocalRendererChanged(bool value)
    {
        _hasLocalRenderer = value;
    }

    // ffmpeg -framerate 15 -probesize 5MB -video_size 1920x1080 -f x11grab -i :0.0
    // -vcodec libx264 -preset veryfast -tune zerolatency -pix_fmt yuv420p -x264opts
    // crf=10:vbv-maxrate=8000:vbv-bufsize=1000:intra-refresh=1:slice-max-size=1500:keyint=30:ref=1
    // -f mpegts tcp://172.16.2.22:50000

    // Ref - https://sites.google.com/site/linuxencoding/x264-ffmpeg-mapping
    bool Encoder::initEncoder(QSize resolution, qint32 rate, qint32 bitrate)
    {
        if ( _initialised )
          return _initialised;

        if ( _codec == NULL )
          _codec = avcodec_find_encoder( AV_CODEC_ID_H264 );

        if ( _codec == NULL )
          return false;

        qDebug() <<  "Codec configuration: " << avcodec_configuration();

        _rate = rate;
        _bitrate = bitrate;
        _resolution = resolution;

        _initialised = true;

        return _initialised;
    }

	// This is called on the first frame received
    bool Encoder::initCodecContext(AVPixelFormat format)
    {
		if (_configured)
			return _configured;

        if ( _codecContext == NULL )
          _codecContext = avcodec_alloc_context3( _codec );

        if ( _codecContext == NULL )
          return false;

        unsigned threadsSupported = 1;

#ifdef Q_OS_WIN
        threadsSupported = std::thread::hardware_concurrency();
        if (threadsSupported == 0)
        {
            threadsSupported = 1;
            qWarning() << "ERROR, checking thread concurrency didn't work and defaulting to 1.  This will impact performance.";
        }
#endif
        _codecContext->thread_count = NumberOfThreads(_resolution.width(),
                                                      _resolution.height(),
                                                      threadsSupported);

        //vbv-maxrate=8000
        _codecContext->rc_max_rate = _bitrate * 1000;
        //vbv-bufsize=1000
        _codecContext->rc_buffer_size = 1000 * 1000;
        //ref=1
        _codecContext->refs = 1;

        _codecContext->width = _resolution.width();
        _codecContext->height = _resolution.height();

        _codecContext->time_base.num = 1;
        _codecContext->time_base.den = _rate;

        // keyint=30;
        _codecContext->gop_size = 3000;
        // -pix_fmt nv12 or yuv420p
		if (format == AV_PIX_FMT_BGRA)
		{
			// We scale to YUV420p if we get RGB32 frames
			_codecContext->pix_fmt = AV_PIX_FMT_YUV420P;
		}
		else
		{
			// This will be NV12
			_codecContext->pix_fmt = format;
		}

        // X264 options
        AVDictionary * codec_options(0);
        // -preset veryfast
        av_dict_set(&codec_options, "preset", "superfast", 0);
        // -tune zerolatency
        av_dict_set(&codec_options, "tune", "zerolatency", 0);
        // crf=10
        av_opt_set_double(_codecContext->priv_data, "crf", 10, 0);
        // intra-refresh=1
        av_opt_set_int(_codecContext->priv_data, "intra-refresh", 1, 0);
        // slice-max-size=1500
        av_opt_set_int(_codecContext->priv_data, "slice-max-size", 1500 * 1000, 0);

        if ( avcodec_open2( _codecContext, _codec, &codec_options ) < 0 )
        {
            qWarning() << "Unable to open codec";
            return false;
        }

        QString log = QString("Codec initialised: %1 x %2 @ %3 fps %4 GOP %5 Kbps")
                .arg(_resolution.width())
                .arg(_resolution.height())
                .arg(_rate)
                .arg(_codecContext->gop_size)
                .arg(_codecContext->rc_max_rate);

        MyApp->message(log);

		_configured = true;
        return _configured;
    }

    /**
     * qint32 is not enough to store millis from epoch, so to measure time taken we cut
     * the millis time to just seconds and millis.
     *
     * @brief Encoder::getMillis
     * @return
     */
    qint32 Encoder::getMillis()
    {
        return App::instance()->getMillis();
    }

    void Encoder::deinitEncoder()
    {
        _codec = nullptr;

        if (_codecContext != nullptr) {
            avcodec_close(_codecContext);
            av_free(_codecContext);
        }
        _codecContext = nullptr;

        if (_swsContext != nullptr) {
            sws_freeContext(_swsContext);
        }
        _swsContext = nullptr;
    }

} // namespace Supernova

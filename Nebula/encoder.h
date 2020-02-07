#ifndef SUPERNOVA_ENCODER_H
#define SUPERNOVA_ENCODER_H

#include "supernova_global.h"
#include "encodedframe.h"
#include "capturedframe.h"
#include <QObject>
#include <QTimer>
#include <QMutex>

namespace Supernova {

    class SUPERNOVASHARED_EXPORT Encoder : public QObject
    {
        Q_OBJECT
    public:
		Encoder(QObject * parent = nullptr);
        virtual ~Encoder();
        bool init(const QSize & resolution,
                  qint32 framerate = DEFAULT_FRAMERATE,
                  qint32 bitrate = DEFAULT_BITRATE,
                  EncodedFrameHeader::Codec codec = EncodedFrameHeader::H264_X264);
        void deinit();

    public Q_SLOTS:
        void capturedFrame(CapturedFrame * frame);
        void encodeFrame();
        bool setBitRate(qint32 bitrate);
        bool setFrameRate(qint32 framerate);
        void onHasLocalRendererChanged(bool value);

    Q_SIGNALS:
        void frameEncoded(EncodedFrame * frame);
        void newLocalFrame(DecodedFrame * frame);

    private:
        EncodedFrameHeader::Codec _frameCodec;
        QMutex _frameMutex;
        CapturedFrame * _capturedFrame;
        QTimer _timer;

        //FFMPEG
        bool initEncoder(QSize resolution, qint32 rate, qint32 bitrate);
        void deinitEncoder();
        bool initCodecContext(AVPixelFormat format);
        qint32 _rate;
        qint32 _bitrate;
        QSize _resolution;
        qint32 getMillis();

        bool _initialised;
		bool _configured;
        bool _hasLocalRenderer;
        AVCodec * _codec;
        AVCodecContext * _codecContext;
        SwsContext * _swsContext;

        QTimer _captureStatsTimer;
        int _captures;
        qint64 _lastTime;
    };

} // namespace Supernova

#endif // SUPERNOVA_ENCODER_H

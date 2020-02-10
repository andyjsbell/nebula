extern crate ffmpeg4_ffi;
extern crate num_cpus;

use std::ffi::{CString, CStr};
use std::{ptr, thread};

use ffmpeg4_ffi::sys::{
    self,
    AVFrame,
    AVDictionary,
    AVCodec,
    AVCodecContext,
    AVStream,
    AVPacket,
    AVFormatContext,
    AVOutputFormat,
    AVCodecParameters,
    AVCodecParserContext,
    AVMediaType,
    AVMediaType_AVMEDIA_TYPE_UNKNOWN as AVMEDIA_TYPE_UNKNOWN,
    AVMediaType_AVMEDIA_TYPE_VIDEO as AVMEDIA_TYPE_VIDEO,
    AVMediaType_AVMEDIA_TYPE_AUDIO as AVMEDIA_TYPE_AUDIO,
    AVMediaType_AVMEDIA_TYPE_DATA as AVMEDIA_TYPE_DATA,
    AVMediaType_AVMEDIA_TYPE_SUBTITLE as AVMEDIA_TYPE_SUBTITLE,
    AVMediaType_AVMEDIA_TYPE_ATTACHMENT as AVMEDIA_TYPE_ATTACHMENT,
    AVMediaType_AVMEDIA_TYPE_NB as AVMEDIA_TYPE_NB,
    AVFMT_NOFILE,
    AVIO_FLAG_WRITE,
    AVRounding_AV_ROUND_NEAR_INF as AV_ROUND_NEAR_INF,
    AVRounding_AV_ROUND_PASS_MINMAX as AV_ROUND_PASS_MINMAX,
    AVCodecID_AV_CODEC_ID_H264 as AV_CODEC_ID_H264,
    AV_INPUT_BUFFER_PADDING_SIZE,
};

fn number_of_threads(width: u32, height: u32, number_of_cores:u8) -> i32 {
    if width * height >= 1920 * 1080 && number_of_cores > 8 {
        return 8;  // 8 threads for 1080p on high perf machines.
    } else if width * height > 1280 * 960 && number_of_cores >= 6 {
        return 3;  // 3 threads for 1080p.
    } else if width * height > 640 * 480 && number_of_cores >= 3 {
        return 2;  // 2 threads for qHD/HD.
    } else {
        return 1;  // 1 thread for VGA or less.
    }
}

pub struct Encoder {
    codec: *mut AVCodec,
    codec_context: * mut AVCodecContext,
}

impl Encoder {
    
    pub fn new() -> Encoder {
        Encoder {
            codec: std::ptr::null_mut(),
            codec_context: std::ptr::null_mut(),
        }
    }

    pub fn initialise(&mut self) -> bool {
        
        unsafe {
            let version = CStr::from_ptr(ffmpeg4_ffi::sys::av_version_info());
            println!("ffmpeg version: {}", version.to_string_lossy().into_owned());

            let configuration = CStr::from_ptr(ffmpeg4_ffi::sys::avcodec_configuration());
            println!("configuration: {}", configuration.to_string_lossy().into_owned());
        
            self.codec = ffmpeg4_ffi::sys::avcodec_find_encoder(ffmpeg4_ffi::sys::AVCodecID_AV_CODEC_ID_H264);
            let codec_name = CStr::from_ptr((*self.codec).name);
            println!("loaded codec: {}", codec_name.to_string_lossy().into_owned());
        
            self.codec_context = ffmpeg4_ffi::sys::avcodec_alloc_context3(self.codec);
        
            (*self.codec_context).rc_max_rate = 5000 * 1000;
            //vbv-bufsize=1000
            (*self.codec_context).rc_buffer_size = 1000 * 1000;
            //ref=1
            (*self.codec_context).refs = 1;

            (*self.codec_context).width = 1920;
            (*self.codec_context).height = 1080;
            (*self.codec_context).time_base.num = 1;
            (*self.codec_context).time_base.den = 30;

            // keyint=30;
            (*self.codec_context).gop_size = 3000;

            let number_of_cores = num_cpus::get() as u8;
            (*self.codec_context).thread_count = number_of_threads(1920, 1080, number_of_cores);
            
            // AV_PIX_FMT_BGRA - capture without NV12 - TODO
            // {
            // 	// We scale to YUV420p if we get RGB32 frames
            // 	(*codecContext).pix_fmt = AVPixelFormat_AV_PIX_FMT_YUV420P;
            // }
            
            // NV12
            (*self.codec_context).pix_fmt = ffmpeg4_ffi::sys::AVPixelFormat_AV_PIX_FMT_NV12;
            
            // // X264 options
            let mut codec_options: *mut AVDictionary = ptr::null_mut();

            // // -preset veryfast
            sys::av_dict_set(  &mut codec_options, 
                                CString::new("preset").unwrap().as_ptr(), 
                                CString::new("superfast").unwrap().as_ptr(), 0);
            // -tune zerolatency
            sys::av_dict_set(  &mut codec_options, 
                                CString::new("tune").unwrap().as_ptr(), 
                                CString::new("zerolatency").unwrap().as_ptr(), 0);
            
            // crf=10
            sys::av_opt_set_double(    (*self.codec_context).priv_data, 
                                        CString::new("crf").unwrap().as_ptr(), 
                                        10 as f64, 0);

            // intra-refresh=1
            sys::av_opt_set_int(   (*self.codec_context).priv_data, 
                                    CString::new("intra-refresh").unwrap().as_ptr(), 
                                    1, 0);

            // slice-max-size=1500
            sys::av_opt_set_int(   (*self.codec_context).priv_data, 
                                    CString::new("slice-max-size").unwrap().as_ptr(), 
                                    1500 * 1000, 0);

            let opened = sys::avcodec_open2(self.codec_context, self.codec, &mut codec_options);
            
            if opened < 0 {
                println!("Failed to open codec ;-(");
            } else {
                println!("Opened codec :-)");
            }
        }
        
        return true;
    }

    pub fn encode_frame(&mut self) {

    }
}


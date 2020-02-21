
pub const TYPE_AVC1: [u8; 4] = [ 97, 118, 99, 49 ];
pub const TYPE_AVCC: [u8; 4] = [ 97, 118, 99, 67 ];
pub const TYPE_BTRT: [u8; 4] = [ 98, 116, 114, 116 ];
pub const TYPE_DINF: [u8; 4] = [ 100, 105, 110, 102 ];
pub const TYPE_DREF: [u8; 4] = [ 100, 114, 101, 102 ];
pub const TYPE_ESDS: [u8; 4] = [ 101, 115, 100, 115 ];
pub const TYPE_FTYP: [u8; 4] = [ 102, 116, 121, 112 ];
pub const TYPE_HDLR: [u8; 4] = [ 104, 100, 108, 114 ];
pub const TYPE_MDAT: [u8; 4] = [ 109, 100, 97, 116 ];
pub const TYPE_MDHD: [u8; 4] = [ 109, 100, 104, 100 ];
pub const TYPE_MDIA: [u8; 4] = [ 109, 100, 105, 97 ];
pub const TYPE_MFHD: [u8; 4] = [ 109, 102, 104, 100 ];
pub const TYPE_MINF: [u8; 4] = [ 109, 105, 110, 102 ];
pub const TYPE_MOOF: [u8; 4] = [ 109, 111, 111, 102 ];
pub const TYPE_MOOV: [u8; 4] = [ 109, 111, 111, 118 ];
pub const TYPE_MP4A: [u8; 4] = [ 109, 112, 52, 97 ];
pub const TYPE_MVEX: [u8; 4] = [ 109, 118, 101, 120 ];
pub const TYPE_MVHD: [u8; 4] = [ 109, 118, 104, 100 ];
pub const TYPE_SDTP: [u8; 4] = [ 115, 100, 116, 112 ];
pub const TYPE_STBL: [u8; 4] = [ 115, 116, 98, 108 ];
pub const TYPE_STCO: [u8; 4] = [ 115, 116, 99, 111 ];
pub const TYPE_STSC: [u8; 4] = [ 115, 116, 115, 99 ];
pub const TYPE_STSD: [u8; 4] = [ 115, 116, 115, 100 ];
pub const TYPE_STSZ: [u8; 4] = [ 115, 116, 115, 122 ];
pub const TYPE_STTS: [u8; 4] = [ 115, 116, 116, 115 ];
pub const TYPE_TFDT: [u8; 4] = [ 116, 102, 100, 116 ];
pub const TYPE_TFHD: [u8; 4] = [ 116, 102, 104, 100 ];
pub const TYPE_TRAF: [u8; 4] = [ 116, 114, 97, 102 ];
pub const TYPE_TRAK: [u8; 4] = [ 116, 114, 97, 107 ];
pub const TYPE_TRUN: [u8; 4] = [ 116, 114, 117, 110 ];
pub const TYPE_TREX: [u8; 4] = [ 116, 114, 101, 120 ];
pub const TYPE_TKHD: [u8; 4] = [ 116, 107, 104, 100 ];
pub const TYPE_VMHD: [u8; 4] = [ 118, 109, 104, 100 ];
pub const TYPE_SMHD: [u8; 4] = [ 115, 109, 104, 100 ];

pub const MAJOR_BRAND: [u8; 4] = [105, 115, 111, 109]; 
pub const AVC1_BRAND: [u8; 4] = [97, 118, 99, 49];
pub const MINOR_VESION: [u8; 4] = [0, 0, 0, 1]; 
pub const VIDEO_HDLR : [u8; 37] = [
    0x00, // version 0
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // pre_defined
    0x76, 0x69, 0x64, 0x65, // handler_type: 'vide'
    0x00, 0x00, 0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, // reserved
    0x56, 0x69, 0x64, 0x65,
    0x6f, 0x48, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x72, 0x00, // name: 'VideoHandler'
];

pub const AUDIO_HDLR : [u8; 37] = [
    0x00, // version 0
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // pre_defined
    0x73, 0x6f, 0x75, 0x6e, // handler_type: 'soun'
    0x00, 0x00, 0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, // reserved
    0x53, 0x6f, 0x75, 0x6e,
    0x64, 0x48, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x72, 0x00, // name: 'SoundHandler'
]; 

pub const SMHD : [u8; 8] = [
    0x00, // version
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, // balance
    0x00, 0x00, // reserved
];

pub const DREF : [u8; 20]  =
[
    0x00, // version 0
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x01, // entry_count
    0x00, 0x00, 0x00, 0x0c, // entry_size
    0x75, 0x72, 0x6c, 0x20, // 'url' type
    0x00, // version 0
    0x00, 0x00, 0x01, // entry_flags
];

pub const STCO : [u8; 8] = [
    0x00, // version
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // entry_count
];

pub const STTS : [u8; 8] = [
    0x00, // version
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // entry_count
];

pub const STSC : [u8; 8] = [
    0x00, // version
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // entry_count
];

pub const STSZ : [u8; 12] = [
    0x00, // version
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // sample_size
    0x00, 0x00, 0x00, 0x00, // sample_count
];

pub const VMHD : [u8; 12] = [
    0x00, // version
    0x00, 0x00, 0x01, // flags
    0x00, 0x00, // graphicsmode
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00, // opcolor
];

pub const STSD : [u8; 8] = [
    0x00, // version 0
    0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x01
];

pub fn dinf() -> Vec<u8> {
    create_box(TYPE_DINF, vec![&create_box(TYPE_DREF, vec![&[
        0x00, // version 0
        0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x01, // entry_count
        0x00, 0x00, 0x00, 0x0c, // entry_size
        0x75, 0x72, 0x6c, 0x20, // 'url' type
        0x00, // version 0
        0x00, 0x00, 0x01, // entry_flags
    ]])])
}

pub fn ftyp() -> Vec<u8> {
    create_box(TYPE_FTYP, 
                vec![   &MAJOR_BRAND, 
                        &MINOR_VESION, 
                        &MAJOR_BRAND, 
                        &AVC1_BRAND])
}

pub fn create_box(mp4_type: [u8; 4], payload: Vec<&[u8]>) -> Vec<u8> {
    let mut size: usize = 8;
    let mut v:Vec<u8> = vec![0];

    for item in payload {
        size = size + item.len();
        v.extend(item);
    }

    let mut result : Vec<u8> = vec![0; size];
    result.push((size >> 24) as u8 & 0xff);
    result.push((size >> 16) as u8 & 0xff);
    result.push((size >> 8) as u8 & 0xff);
    result.push(size as u8 & 0xff);
    result.extend_from_slice(&mp4_type);
    result.append(&mut v);
    
    result
}

pub fn hdlr(mp4_type: [u8; 37]) -> Vec<u8> {
    
    create_box(TYPE_HDLR, vec![&mp4_type])    
}

pub fn mdat(data: [u8; 4]) -> Vec<u8> {
    create_box(TYPE_MDAT, vec![&data])
}

// pub fn stbl(track) {
//     // create_box()
//     // return MP4.box(MP4.types.stbl, MP4.stsd(track), MP4.box(MP4.types.stts, MP4.STTS), MP4.box(MP4.types.stsc, MP4.STSC), MP4.box(MP4.types.stsz, MP4.STSZ), MP4.box(MP4.types.stco, MP4.STCO));
// }

pub struct Track {
    pub timescale: u32,
    pub duration: u32,
    pub track_type: String,
    pub width: u32,
    pub height: u32,
    pub volume: u32,
    pub id: u32,
    pub audio_sample_rate: u32,
    pub channel_count: u32,
    pub config: Vec<u8>,
}

pub fn mdhd(timescale: u32, duration: u32) -> Vec<u8> {
    create_box(TYPE_MDHD, vec![&[
        0x00, // version 0
        0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x02, // creation_time
        0x00, 0x00, 0x00, 0x03, // modification_time
        (timescale >> 24) as u8 & 0xFF,
        (timescale >> 16) as u8 & 0xFF,
        (timescale >> 8) as u8 & 0xFF,
        timescale as u8 & 0xFF, // timescale
        (duration >> 24) as u8,
        (duration >> 16) as u8 & 0xFF,
        (duration >> 8) as u8 & 0xFF,
        duration as u8 & 0xFF, // duration
        0x55, 0xc4, // 'und' language (undetermined)
        0x00, 0x00,
    ]])
}

pub fn mdia(track: &Track) -> Vec<u8> {
    let t = if track.track_type == "video" {VIDEO_HDLR} else {AUDIO_HDLR};

    create_box(TYPE_MDIA, vec![
        &mdhd(track.timescale, track.duration),
        &hdlr(t),
        &minf(track)
    ])
}

pub fn mfhd(sequence_number: u32) -> Vec<u8> {
    create_box(TYPE_MFHD, vec![&[
        0x00,
        0x00, 0x00, 0x00, // flags
        (sequence_number >> 24) as u8,
        (sequence_number >> 16) as u8 & 0xFF,
        (sequence_number >> 8) as u8 & 0xFF,
        sequence_number as u8 & 0xFF, // sequence_number
    ]])
}

pub fn minf(track: &Track) -> Vec<u8> {
    if track.track_type == "audio" {
        create_box(TYPE_MINF, vec![
            &create_box(TYPE_SMHD, SMHD),
            &dinf(),
            stbl(track)
        ])
    } else {
        create_box(TYPE_MINF, vec![
            &create_box(TYPE_VMHD, VMHD),
            &dinf(),
            stbl(track)
        ])
    }
}

pub fn stbl(track: &Track) -> Vec<u8> {
    create_box(TYPE_STBL, vec![
        &st
    ]);
    // return MP4.box(MP4.types.stbl, MP4.stsd(track), MP4.box(MP4.types.stts, MP4.STTS), MP4.box(MP4.types.stsc, MP4.STSC), MP4.box(MP4.types.stsz, MP4.STSZ), MP4.box(MP4.types.stco, MP4.STCO));
}

pub fn stsd(track: &Track) -> Vec<u8> {
    if track.track_type == "audio" {
        create_box(TYPE_STSD, vec![
            &STSD,
            &mp4a(track)
        ])
        // return MP4.box(MP4.types.stsd, MP4.STSD, MP4.mp4a(track));
    } else {
        // return MP4.box(MP4.types.stsd, MP4.STSD, MP4.avc1(track));
        create_box(TYPE_STSD, vec![
            &STSD,
            &avc1(track)
        ])
    }
}

pub fn mp4a(track: &Track) {
    create_box(TYPE_MP4A, vec![
        &[
            0x00, 0x00, 0x00, // reserved
            0x00, 0x00, 0x00, // reserved
            0x00, 0x01, // data_reference_index
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // reserved
            0x00, track.channel_count as u8, // channelcount
            0x00, 0x10, // sampleSize:16bits
            0x00, 0x00, // pre_defined
            0x00, 0x00, // reserved2
            (track.audio_sample_rate >> 8) as u8 & 0xFF,
            track.audio_sample_rate as u8 & 0xff, //
            0x00, 0x00
        ],
        create_box(TYPE_ESDS, vec![&esds(track)])
    ])
}

pub fn esds(track: &Track) -> Vec<u8> {

    let config_length = track.config.len() as u8;
    let v : Vec<u8> =  Vec::new();
    v.extend_from_slice(&[
        0x00, // version 0
        0x00, 0x00, 0x00, // flags

        0x03, // descriptor_type
        0x17 + config_length, // length
        0x00, 0x01, // es_id
        0x00, // stream_priority

        0x04, // descriptor_type
        0x0f + config_length, // length
        0x40, // codec : mpeg4_audio
        0x15, // stream_type
        0x00, 0x00, 0x00, // buffer_size
        0x00, 0x00, 0x00, 0x00, // maxBitrate
        0x00, 0x00, 0x00, 0x00, // avgBitrate

        0x05, // descriptor_type
        config_length,
    ]);
    
    v.append(track.config);
    v.extend_from_slice(&[0x06, 0x01, 0x02]);
    
    v
}

pub fn moof(sequence_number: u32, base_media_decode_time: u32, track: Track) -> Vec<u8> {
    // return MP4.box(MP4.types.moof, MP4.mfhd(sn), MP4.traf(track, baseMediaDecodeTime));
    vec![0]
}

pub fn moov(tracks: Vec<Track>, duration: u32, timescale: u32) -> Vec<u8> {
    // var
    //     i = tracks.length,
    //     boxes = [];

    // while (i--) {
    //     boxes[i] = MP4.trak(tracks[i]);
    // }

    // return MP4.box.apply(null, [MP4.types.moov, MP4.mvhd(timescale, duration)].concat(boxes).concat(MP4.mvex(tracks)));
    vec![0]
}

pub fn trak(track: &mut Track) -> Vec<u8> {
    if track.duration == 0 {
        track.duration = 0xffffffff;
    }
    create_box(TYPE_TRAK, vec![&tkhd(&track), &mdia(&track)])
}

pub fn tkhd(track: &Track) -> Vec<u8> {
    create_box(TYPE_TKHD, vec![&[
        0x00, // version 0
        0x00, 0x00, 0x07, // flags
        0x00, 0x00, 0x00, 0x00, // creation_time
        0x00, 0x00, 0x00, 0x00, // modification_time
        (track.id >> 24) as u8 & 0xFF,
        (track.id >> 16) as u8 & 0xFF,
        (track.id >> 8) as u8 & 0xFF,
        track.id as u8 & 0xFF, // track_ID
        0x00, 0x00, 0x00, 0x00, // reserved
        (track.duration >> 24) as u8 & 0xFF,
        (track.duration >> 16) as u8 & 0xFF,
        (track.duration >> 8) as u8 & 0xFF,
        track.duration as u8 & 0xFF, // duration
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, // reserved
        0x00, 0x00, // layer
        0x00, 0x00, // alternate_group
        (track.volume >> 0) as u8 & 0xff, (((track.volume % 1) * 10) >> 0) as u8 & 0xff, // track volume // FIXME
        0x00, 0x00, // reserved
        0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00, // transformation: unity matrix
        (track.width >> 8) as u8 & 0xFF,
        track.width as u8 & 0xFF,
        0x00, 0x00, // width
        (track.height >> 8) as u8 & 0xFF,
        track.height as u8 & 0xFF,
        0x00, 0x00, // height
    ]])
}
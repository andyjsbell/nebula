
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

#[derive(Clone)]
pub struct Flags {
    pub is_leading: u8,
    pub is_depended_on: u8,
    pub has_redundancy: u8,
    pub degrad_prio: u8,
    pub is_non_sync: u8,
    pub depends_on: u8,
    pub padding_value: u8,
}

#[derive(Clone)]
pub struct Sample {
    pub size: u32,
    pub duration: u32,
    pub cts: u32,
    pub flags: Flags,
}

#[derive(Clone)]
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
    pub codec: String,
    pub pps : Vec<u8>,
    pub sps: Vec<u8>,
    pub samples : Vec<Sample>,
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
            &create_box(TYPE_SMHD, vec![&SMHD]),
            &dinf(),
            &stbl(track)
        ])
    } else {
        create_box(TYPE_MINF, vec![
            &create_box(TYPE_VMHD, vec![&VMHD]),
            &dinf(),
            &stbl(track)
        ])
    }
}

pub fn stbl(track: &Track) -> Vec<u8> {
    create_box(TYPE_STBL, vec![
        &stsd(track),
        &create_box(TYPE_STTS, vec![&STTS]),
        &create_box(TYPE_STSC, vec![&STSC]),
        &create_box(TYPE_STSZ, vec![&STSZ]),
        &create_box(TYPE_STCO, vec![&STCO]),        
    ])
}

pub fn stsd(track: &Track) -> Vec<u8> {
    if track.track_type == "audio" {
        create_box(TYPE_STSD, vec![
            &STSD,
            &mp4a(track)
        ])
    } else {
        create_box(TYPE_STSD, vec![
            &STSD,
            &avc1(track)
        ])
    }
}

pub fn mp4a(track: &Track) -> Vec<u8> {
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
        &create_box(TYPE_ESDS, vec![&esds(track)])
    ])
}

pub fn avc1(track: &Track) -> Vec<u8> {
    
    let mut sps = Vec::new();
    let mut pps = Vec::new();
    
    sps.push((track.sps.len() >> 8) as u8 & 0xff);
    sps.push(track.sps.len() as u8 & 0xff);
    sps.extend_from_slice(&track.sps);

    pps.push((track.pps.len() >> 8) as u8 & 0xff);
    pps.push(track.pps.len() as u8 & 0xff);
    pps.extend_from_slice(&track.pps);

    let mut p  = vec![
        0x01,   // version
        sps[3], // profile
        sps[4], // profile compat
        sps[5], // level
        0xfc | 3, // lengthSizeMinusOne, hard-coded to 4 bytes
        0xE0 | track.sps.len() as u8, // 3bit reserved (111) + numOfSequenceParameterSets
    ];
    p.extend_from_slice(&sps);
    p.push(track.pps.len() as u8);
    p.extend_from_slice(&pps);
    
    let avcc = create_box(TYPE_AVCC, vec![&p]);
        
    create_box(TYPE_AVCC, vec![&[
        0x00, 0x00, 0x00, // reserved
        0x00, 0x00, 0x00, // reserved
        0x00, 0x01, // data_reference_index
        0x00, 0x00, // pre_defined
        0x00, 0x00, // reserved
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, // pre_defined
        (track.width >> 8) as u8 & 0xFF,
        track.width as u8 & 0xff, // width
        (track.height >> 8) as u8 & 0xFF,
        track.height as u8 & 0xff, // height
        0x00, 0x48, 0x00, 0x00, // horizresolution
        0x00, 0x48, 0x00, 0x00, // vertresolution
        0x00, 0x00, 0x00, 0x00, // reserved
        0x00, 0x01, // frame_count
        0x12,
        0x62, 0x69, 0x6E, 0x65, // binelpro.ru
        0x6C, 0x70, 0x72, 0x6F,
        0x2E, 0x72, 0x75, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, // compressorname
        0x00, 0x18,   // depth = 24
        0x11, 0x11], // pre_defined = -1
        &avcc,
        &create_box(TYPE_BTRT, vec![&[
                    0x00, 0x1c, 0x9c, 0x80, // bufferSizeDB
                    0x00, 0x2d, 0xc6, 0xc0, // maxBitrate
                    0x00, 0x2d, 0xc6, 0xc0]]) // avgBitrate
    ])
}

pub fn esds(track: &Track) -> Vec<u8> {

    let config_length = track.config.len() as u8;
    let mut v : Vec<u8> =  Vec::new();
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
    
    v.extend_from_slice(&track.config);
    v.extend_from_slice(&[0x06, 0x01, 0x02]);
    
    v
}

pub fn trun(track: &Track, offset: u32) -> Vec<u8> {
    
    let len = track.samples.len() as u32;
    let array_len = 12 + (16 * len);
    let off : u32 = offset + 8 + array_len;

    let mut v = vec![
        0x00, // version 0
        0x00, 0x0f, 0x01, // flags
        (len >> 24) & 0xFF,
        (len >> 16) & 0xFF,
        (len >> 8) & 0xFF,
        len & 0xFF, // sample_count
        (off >> 24) as u8 & 0xFF,
        (off >> 16) as u8 & 0xFF,
        (off >> 8) as u8 & 0xFF,
        off & 0xFF, // data_offset
    ];
    
    for sample in track.samples {
        v.extend_from_slice([
            (sample.duration >> 24) & 0xFF,
            (sample.duration >> 16) & 0xFF,
            (sample.duration >> 8) & 0xFF,
            sample.duration & 0xFF, // sample_duration
            (sample.size >> 24) & 0xFF,
            (sample.size >> 16) & 0xFF,
            (sample.size >> 8) & 0xFF,
            sample.size & 0xFF, // sample_size
            ((sample.flags.is_leading << 2) as u8 | sample.flags.depends_on as u8).into(),
            (sample.flags.is_depended_on << 6) as u8 |
            (sample.flags.has_redundancy << 4) as u8 |
            (sample.flags.padding_value << 1) as u8 |
            sample.flags.is_non_sync as u8,
            sample.flags.degrad_prio & 0xF0 << 8,
            sample.flags.degrad_prio & 0x0F, // sample_flags
            (sample.cts >> 24) & 0xFF,
            (sample.cts >> 16) & 0xFF,
            (sample.cts >> 8) & 0xFF,
            sample.cts & 0xFF, // sample_composition_time_offset
        ]);
    }

    create_box(TYPE_TRUN, vec![&v])
}

pub fn moof(sequence_number: u32, base_media_decode_time: u32, track: &Track) -> Vec<u8> {
    create_box(TYPE_MOOF, vec![&mfhd(sequence_number), &traf(track, base_media_decode_time)])
}

pub fn sdtp(track: &Track) -> Vec<u8> {
    
    let mut v = Vec::new();

    for sample in track.samples {
        let f = sample.flags;
        v.push(f.depends_on << 4);
        v.push(f.is_depended_on << 2);
        v.push(f.has_redundancy);
    }
    
    create_box(TYPE_SDTP, vec![&v])
}

pub fn traf(track: &Track, base_media_decode_time: u32) -> Vec<u8> {
    
    let sample_dependency_table = sdtp(track);

    create_box(TYPE_TRAF, vec![
        &create_box(TYPE_TFHD, vec![&[
            0x00, // version 0
            0x00, 0x00, 0x00, // flags
            (track.id >> 24) as u8,
            (track.id >> 16) as u8 & 0xff,
            (track.id >> 8) as u8 & 0xff,
            (track.id & 0xFF) as u8 , // track_ID
        ]]),
        &create_box(TYPE_TFDT, vec![&[
            0x00, // version 0
            0x00, 0x00, 0x00, // flags
            (base_media_decode_time >> 24) as u8,
            (base_media_decode_time >> 16) as u8 & 0xff,
            (base_media_decode_time >> 8) as u8 & 0xff,
            (base_media_decode_time as u8 & 0xFF), // baseMediaDecodeTime
        ]]), 
        &trun(track, sample_dependency_table.len() as u32 +
        16 + // tfhd
        16 + // tfdt
        8 +  // traf header
        16 + // mfhd
        8 +  // moof header
        8),
        &sample_dependency_table,       
    ])
}

pub fn trak(track: &mut Track) -> Vec<u8> {
    if track.duration == 0 {
        track.duration = 0xffffffff;
    }
    create_box(TYPE_TRAK, vec![&tkhd(&track), &mdia(&track)])
}

pub fn moov(tracks: Vec<Track>, duration: u32, timescale: u32) -> Vec<u8> {

    let mut boxes = Vec::new();
    let mut reversed_tracks = tracks.clone();
    reversed_tracks.reverse();
    for track in reversed_tracks {
        boxes.extend_from_slice(&(trak(&mut track)));
    }

    vec![0]
    // return MP4.box.apply(null, [MP4.types.moov, MP4.mvhd(timescale, duration)].concat(boxes).concat(MP4.mvex(tracks)));
}

pub fn mvex(tracks: Vec<Track>) -> Vec<u8> {

    let mut boxes = Vec::new();
    let mut reversed_tracks = tracks.clone();
    reversed_tracks.reverse();
    for track in reversed_tracks {
        boxes.extend_from_slice(&(trex(&mut track)));
    }

    vec![0]
    // return MP4.box.apply(null, [MP4.types.mvex].concat(boxes));
}

pub fn trex(track: &Track) -> Vec<u8> {
    
    create_box(TYPE_TREX, vec![&[
        0x00, // version 0
        0x00, 0x00, 0x00, // flags
        (track.id >> 24) as u8,
        (track.id >> 16) as u8 & 0xff,
        (track.id >> 8) as u8 & 0xff,
        (track.id as u8 & 0xFF), // track_ID
        0x00, 0x00, 0x00, 0x01, // default_sample_description_index
        0x00, 0x00, 0x00, 0x00, // default_sample_duration
        0x00, 0x00, 0x00, 0x00, // default_sample_size
        0x00, 0x01, 0x00, 0x01, // default_sample_flags
    ]])
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

pub fn init_segment(tracks: Vec<Track>, duration: u32, timescale: u32) -> Vec<u8> {
    
    let v = Vec::new();
    v.extend_from_slice(&ftyp());
    v.extend_from_slice(&moov(tracks, duration, timescale));
    v
}

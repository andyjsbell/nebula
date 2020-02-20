
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

pub struct MP4 {
    pub video_hdlr : [u8; 37],
    pub audio_hdlr : [u8; 37],
    pub dref: [u8; 20],
    pub stco: [u8; 8],
    pub stts: [u8; 8],
    pub stsc: [u8; 8],
    pub stsz: [u8; 12],
    pub vmhd: [u8; 12],
    pub smhd: [u8; 8],
    pub stsd: [u8; 8],    
    pub ftyp: Vec<u8>,
    pub dinf: Vec<u8>,
}

impl Default for MP4 {
    fn default() -> MP4 {
        MP4 {
            video_hdlr: [
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
            ],
            audio_hdlr: [
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
            ],
            dref: [
                0x00, // version 0
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x01, // entry_count
                0x00, 0x00, 0x00, 0x0c, // entry_size
                0x75, 0x72, 0x6c, 0x20, // 'url' type
                0x00, // version 0
                0x00, 0x00, 0x01, // entry_flags
            ],
            stco: [
                0x00, // version
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x00, // entry_count
            ],
            stts: [
                0x00, // version
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x00, // entry_count
            ],
            stsc: [
                0x00, // version
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x00, // entry_count
            ],
            stsz: [
                0x00, // version
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x00, // sample_size
                0x00, 0x00, 0x00, 0x00, // sample_count
            ],
            vmhd: [
                0x00, // version
                0x00, 0x00, 0x01, // flags
                0x00, 0x00, // graphicsmode
                0x00, 0x00,
                0x00, 0x00,
                0x00, 0x00, // opcolor
            ],
            smhd: [
                0x00, // version
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, // balance
                0x00, 0x00, // reserved
            ],
            stsd: [
                0x00, // version 0
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x01
            ],
            ftyp: create_box(TYPE_FTYP, 
                            vec![   &MAJOR_BRAND, 
                                    &MINOR_VESION, 
                                    &MAJOR_BRAND, 
                                    &AVC1_BRAND]),

            dinf: create_box(TYPE_DINF, vec![&create_box(TYPE_DREF, vec![&[
                0x00, // version 0
                0x00, 0x00, 0x00, // flags
                0x00, 0x00, 0x00, 0x01, // entry_count
                0x00, 0x00, 0x00, 0x0c, // entry_size
                0x75, 0x72, 0x6c, 0x20, // 'url' type
                0x00, // version 0
                0x00, 0x00, 0x01, // entry_flags
            ]])]),
        }
    }
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

pub fn hdlr(mp4_type: [u8; 4]) -> Vec<u8> {
    
    create_box(TYPE_HDLR, vec![&mp4_type])    
}

pub fn mdat(data: [u8; 4]) -> Vec<u8> {
    create_box(TYPE_MDAT, vec![&data])
}

pub struct Track {
    pub timescale: u32,
    pub duration: u32,
    pub track_type: &str,
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

pub fn mdia(track: Track, mp4: MP4) -> Vec<u8> {
    let t = if track.track_type == "video" {mp4.video_hdlr} else {mp4.audio_hdlr};
    vec![]
    // create_box(TYPE_MDIA, vec![&mdhd(track.timescale, track.duration), &hdlr(t), ])
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

pub fn minf(track: &str) -> Vec<u8> {
    // if (track.type === 'audio') {
    //     return MP4.box(MP4.types.minf, MP4.box(MP4.types.smhd, MP4.SMHD), MP4.DINF, MP4.stbl(track));
    // } else {
    //     return MP4.box(MP4.types.minf, MP4.box(MP4.types.vmhd, MP4.VMHD), MP4.DINF, MP4.stbl(track));
    // }
    vec![]
}

pub fn moof(sequence_number: u32, baseMediaDecodeTime, track) {
    // return MP4.box(MP4.types.moof, MP4.mfhd(sn), MP4.traf(track, baseMediaDecodeTime));
    vec![]
}

pub fn moov(tracks: Vec<Track>, duration: u32, timescale: u32) -> Vec<u8> {
    var
        i = tracks.length,
        boxes = [];

    while (i--) {
        boxes[i] = MP4.trak(tracks[i]);
    }

    return MP4.box.apply(null, [MP4.types.moov, MP4.mvhd(timescale, duration)].concat(boxes).concat(MP4.mvex(tracks)));
}

pub fn trak(mut track: Track) -> Vec<u8> {
    if track.duration == 0 {
        track.duration = 0xffffffff;
    }
    // create_box(TYPE_TRAK, MP4.tkhd(track), MP4.mdia(track));

    vec![]
}

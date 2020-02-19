

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
    pub major_brand: [u8; 4],
    pub avc1_brand: [u8; 4],
    pub minor_vesion: [u8; 4],
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
            major_brand: [105, 115, 111, 109],
            avc1_brand: [97, 118, 99, 49],
            minor_vesion: [0, 0, 0, 1],
            ftyp: create_box(),
            dinf: create_box(),
        }
    }
}

pub fn create_box() -> Vec<u8> {
    vec![]
}

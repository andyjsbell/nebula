
pub struct Types {
    avc1: [u8; 4], // codingname
    avcC: [u8; 4],
    btrt: [u8; 4],
    dinf: [u8; 4],
    dref: [u8; 4],
    esds: [u8; 4],
    ftyp: [u8; 4],
    hdlr: [u8; 4],
    mdat: [u8; 4],
    mdhd: [u8; 4],
    mdia: [u8; 4],
    mfhd: [u8; 4],
    minf: [u8; 4],
    moof: [u8; 4],
    moov: [u8; 4],
    mp4a: [u8; 4],
    mvex: [u8; 4],
    mvhd: [u8; 4],
    sdtp: [u8; 4],
    stbl: [u8; 4],
    stco: [u8; 4],
    stsc: [u8; 4],
    stsd: [u8; 4],
    stsz: [u8; 4],
    stts: [u8; 4],
    tfdt: [u8; 4],
    tfhd: [u8; 4],
    traf: [u8; 4],
    trak: [u8; 4],
    trun: [u8; 4],
    trex: [u8; 4],
    tkhd: [u8; 4],
    vmhd: [u8; 4],
    smhd: [u8; 4],
}
impl Default for Types {
    fn default() -> Types {
        Types {
            avc1: [ 97, 118, 99, 49 ],
            avcC: [ 97, 118, 99, 67 ],
            btrt: [ 98, 116, 114, 116 ],
            dinf: [ 100, 105, 110, 102 ],
            dref: [ 100, 114, 101, 102 ],
            esds: [ 101, 115, 100, 115 ],
            ftyp: [ 102, 116, 121, 112 ],
            hdlr: [ 104, 100, 108, 114 ],
            mdat: [ 109, 100, 97, 116 ],
            mdhd: [ 109, 100, 104, 100 ],
            mdia: [ 109, 100, 105, 97 ],
            mfhd: [ 109, 102, 104, 100 ],
            minf: [ 109, 105, 110, 102 ],
            moof: [ 109, 111, 111, 102 ],
            moov: [ 109, 111, 111, 118 ],
            mp4a: [ 109, 112, 52, 97 ],
            mvex: [ 109, 118, 101, 120 ],
            mvhd: [ 109, 118, 104, 100 ],
            sdtp: [ 115, 100, 116, 112 ],
            stbl: [ 115, 116, 98, 108 ],
            stco: [ 115, 116, 99, 111 ],
            stsc: [ 115, 116, 115, 99 ],
            stsd: [ 115, 116, 115, 100 ],
            stsz: [ 115, 116, 115, 122 ],
            stts: [ 115, 116, 116, 115 ],
            tfdt: [ 116, 102, 100, 116 ],
            tfhd: [ 116, 102, 104, 100 ],
            traf: [ 116, 114, 97, 102 ],
            trak: [ 116, 114, 97, 107 ],
            trun: [ 116, 114, 117, 110 ],
            trex: [ 116, 114, 101, 120 ],
            tkhd: [ 116, 107, 104, 100 ],
            vmhd: [ 118, 109, 104, 100 ],
            smhd: [ 115, 109, 104, 100 ],
        }
    }
}

pub struct Constants {
    pub major_brand: [u8; 4],
    pub avc1_brand: [u8; 4],
    pub minor_vesion: [u8; 4],
}
impl Default for Constants {
    fn default() -> Constants {
        Constants {
            major_brand: [105, 115, 111, 109],
            avc1_brand: [97, 118, 99, 49],
            minor_vesion: [0, 0, 0, 1],
        }
    }
}
const MP4_TYPES: Types = Types::default();
const MP4_CONST: Constants = Constants::default();

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
            ftyp: vec![],
            dinf: vec![],
            // ftyp: create_box(MP4_TYPES.ftyp, 
            //                 vec![   MP4_CONST.major_brand, 
            //                         MP4_CONST.minor_vesion, 
            //                         MP4_CONST.major_brand, 
            //                         MP4_CONST.avc1_brand]),

            // dinf: create_box(MP4_TYPES.dinf, create_box(MP4_TYPES.dref, dref)),
        }
    }
}

pub fn create_box(mp4_type: [u8; 4], payload: Vec<Vec<u8>>) -> Vec<u8> {
    // let size = 8;
    // let i = payload.len();
    // let len = i;
    // let result = 0;

    // let a: [u8; 4] = [0,0,0,0];
    // let b: [u8; 2] = [0,0];
    
    // let v: Vec<&[u8]> = [&a[..], &b[..]];

}

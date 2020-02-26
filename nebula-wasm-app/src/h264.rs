
pub const NDR : u8 = 1;
pub const IDR : u8= 5;
pub const SEI : u8= 6;
pub const SPS : u8= 7;
pub const PPS : u8= 8;
pub const AUD : u8= 9;

pub struct Nalu {
    pub payload : Vec<u8>,
    pub nri : u8,
    pub ntype: u8,
}

impl Nalu {

    pub fn new(data: Vec<u8>) -> Nalu {
        Nalu {
            nri: (data[0] & 0x60) >> 5,
            ntype: data[0] & 0x1f,
            payload: data,
        }
    }

    pub fn is_keyframe(&self) -> bool {
        self.ntype == IDR
    }

    pub fn get_size(&self ) -> u32 {
        return 4 + self.payload.len() as u32;
    }

}

pub fn parse_to_nalu(data: Vec<u8>) -> Vec<Nalu> {
    let nalus : Vec<Nalu> = Vec::new();

    // for x in data.step_by(4) {
    //     println!("{}", x);
    // }
    nalus
}
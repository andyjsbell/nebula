
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

    let mut nalus : Vec<Nalu> = Vec::new();
    
    let mut append_nalu = |start_index: usize, v: &Vec<u8>, idx: usize, size: usize| {
        let end_index = if idx < v.len() { idx } else { v.len() - 1 };
        println!("append_nalu: {} {} {}", start_index as u32, idx as u32, end_index as u32);
        let unit = &v[start_index + size..end_index];
        let mut v : Vec<u8> = Vec::new();
        v.extend_from_slice(unit);
        let nalu = Nalu::new(v);
        println!("nal added: {:?}", nalu.payload);
        nalus.push(nalu);
    };
   
    let mut start_index : i32 = -1;
   
    for n in 0..data.len() {
        // take 4 bytes from this index and if 0x0000001 then we have a NAL
        let window = if (n + 4) > data.len() {data.len() - 4} else {n + 4};
        println!("window: {}", window);
        let s = &data[n..window];
        println!("{:?}", s);
        match s {
            [_,0,0,1] => {
                println!("nal code: {}", start_index);
                if start_index != -1 {
                    append_nalu(start_index as usize, &data, n, 4);
                } 
                start_index = n as i32;
            },
            _ => (),
        }
    }

    nalus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_nalu() {
        let v : Vec<u8> = vec![0,0,0,1,1,2,3,4,0,0,0,1,5,6,0,0,0,1,7];
        parse_to_nalu(v);
        assert_eq!(3, 3);
    }
}

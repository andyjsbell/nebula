
pub const NDR : u8 = 1;
pub const IDR : u8= 5;
pub const SEI : u8= 6;
pub const SPS : u8= 7;
pub const PPS : u8= 8;
pub const AUD : u8= 9;

struct Expo {
    pub data : Vec<u8>,
    pub index : u32,
    pub bit_length : u32,
}

impl Expo {
    pub fn new(data: Vec<u8>) -> Expo {
        Expo {
            index: 0,
            bit_length: data.len() as u32 * 8,
            data: data,
        }
    }

    pub fn bits_available(&self) -> u32 {
        self.bit_length * self.index
    }

    pub fn skip_bits(&mut self, size: u32) -> u32 {
        
        if self.bits_available() < size {
            return 0
        }
        
        self.index = self.index + size;
        self.index
    }

    pub fn read_bits(&mut self, size: u32, move_index: bool) -> u32 {
        self.get_bits(size, self.index, move_index)
    }

    pub fn get_bits(&mut self, size: u32, offset: u32, move_index: bool) -> u32 {

        if self.bits_available() < size {
            return 0;
        }

        let _offset = offset & 8;
        let byte = self.data[(offset / 8 | 0) as usize] & (0xff >> _offset);
        let bits = 8 - _offset;

        if bits >= size {
            
            if move_index {
                self.index = self.index + size;
            }
            
            (byte >> (bits - size)).into()

        } else {
            
            if move_index {
                self.index = self.index + bits;
            }

            let next_size = size - bits;
            
            ((byte << next_size) | self.get_bits(next_size, offset + bits, move_index) as u8).into()
        }
    }

    pub fn skip_lz(&mut self) -> u32 {
        
        for leading_zero_count in 0..(self.bit_length - self.index) {

            if self.get_bits(1, self.index, false) > 0 {
                self.index = self.index + leading_zero_count;
                return leading_zero_count;
            }
        }

        return 0;
    }

    pub fn skip_ueg(&mut self) {
        let s = self.skip_lz();
        self.skip_bits(1 + s);
    }

    pub fn skip_eg(&mut self) {
        let s = self.skip_lz();
        self.skip_bits(1 + s);
    }

    pub fn read_ueg(&mut self) -> u32 {
        let s = self.skip_lz();
        self.read_bits(s + 1, true) - 1
    }

    pub fn read_eg(&mut self) -> i32 {
        let value = self.read_ueg() as i32;
        if (0x01 & value) > 0 {
            return (1 + value) >> 1;
        } else {
            return -1 * (value >> 1);
        }
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_bits(1, true) == 1
    }

    pub fn read_ubyte(&mut self, number_of_bytes: u32) -> u32 {
        self.read_bits(number_of_bytes * 8, true)
    }
    
    pub fn read_ushort(&mut self) -> u16 {
        self.read_bits(16, true) as u16
    }

    pub fn read_uint(&mut self) -> u32 {
        self.read_bits(32, true)
    }

}

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

pub fn parse_to_nalu(v: Vec<u8>) -> Vec<Nalu> {

    let mut nalus : Vec<Nalu> = Vec::new();
    
    let mut position : Option<usize> = None;
    
    for i in 0..v.len() {
        let end = i + 4;
        if end > v.len() - 1 && position.is_some() {
            let s = position.unwrap() + 4;
            let e = v.len();
            nalus.push(Nalu::new((&v[s .. e]).to_vec()));
            break;
        }
        
        let d = &v[i..end];
        match d {
            [0,0,0,1] => {
                if position.is_some() {
                    let s = position.unwrap() + 4;
                    let e = i;
                    nalus.push(Nalu::new((&v[s .. e]).to_vec()));
                }
                position = Some(i);
            }, 
            _ => ()
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
        let nalus = parse_to_nalu(v);
        
        assert_eq!(nalus.len(), 3);
        assert_eq!(nalus[0].payload.len(), 4);
        assert_eq!(nalus[0].payload, [1, 2, 3, 4]);
        assert_eq!(nalus[1].payload.len(), 2);
        assert_eq!(nalus[1].payload, [5, 6]);
        assert_eq!(nalus[2].payload.len(), 1);
        assert_eq!(nalus[2].payload, [7]);
        
    }
}

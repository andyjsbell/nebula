
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

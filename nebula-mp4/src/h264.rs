use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct SequencePictureSet {
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub sar_scale: u8,
    pub profile_idc: u32,
    pub profile_compat: u32,
    pub level_idc: u32,
    pub num_ref_frames_in_pic_order_cnt_cycle: u32,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,
    pub frame_mbs_only_flag: u32,
    pub scaling_list_count: u32,
    pub units_in_tick: u32,
    pub time_scale: u32,
    pub fixed_frame_rate: bool,
    pub frame_duration: u32,
}

impl SequencePictureSet {
    pub fn new() -> SequencePictureSet{
        SequencePictureSet {
            frame_crop_left_offset: 0,
            frame_crop_right_offset: 0,
            frame_crop_top_offset: 0,
            frame_crop_bottom_offset: 0,
            sar_scale: 1,
            profile_compat: 0,
            profile_idc: 0,
            level_idc: 0,
            num_ref_frames_in_pic_order_cnt_cycle: 0,
            pic_height_in_map_units_minus1: 0,
            pic_width_in_mbs_minus1: 0,
            frame_mbs_only_flag: 0,
            scaling_list_count: 0,
            units_in_tick: 0,
            time_scale: 0,
            fixed_frame_rate: false,
            frame_duration: 0,
        }
    }

    pub fn config(&self) -> (u32, u32) {
        let width = 0;
        let height = 0;
        (width, height)
            // return {
    //     width: Math.ceil((((picWidthInMbsMinus1 + 1) * 16) - frameCropLeftOffset * 2 - frameCropRightOffset * 2) * sarScale),
    //     height: ((2 - frameMbsOnlyFlag) * (picHeightInMapUnitsMinus1 + 1) * 16) - ((frameMbsOnlyFlag ? 2 : 4) * (frameCropTopOffset + frameCropBottomOffset)),
    // };
    }
}

pub fn read_sps(data: &Vec<u8>) -> SequencePictureSet {
    
    let mut decoder = Expo::new(&data);
    decoder.read_ubyte(1);
    let mut sps = SequencePictureSet::new();
    sps.profile_idc = decoder.read_ubyte(1); // profile_idc
    sps.profile_compat = decoder.read_bits(5, true); // constraint_set[0-4]_flag, u(5)
    decoder.skip_bits(3); // reserved_zero_3bits u(3),
    sps.level_idc = decoder.read_ubyte(1); // level_idc u(8)
    decoder.skip_ueg(); // seq_parameter_set_id
    // some profiles have more optional data we don't need
    if sps.profile_idc == 100 ||
        sps.profile_idc == 110 ||
        sps.profile_idc == 122 ||
        sps.profile_idc == 244 ||
        sps.profile_idc == 44 ||
        sps.profile_idc == 83 ||
        sps.profile_idc == 86 ||
        sps.profile_idc == 118 ||
        sps.profile_idc == 128 {
        
        let chroma_format_idc = decoder.read_ueg();
        if chroma_format_idc == 3 {
            decoder.skip_bits(1); // separate_colour_plane_flag
        }
        decoder.skip_ueg(); // bit_depth_luma_minus8
        decoder.skip_ueg(); // bit_depth_chroma_minus8
        decoder.skip_bits(1); // qpprime_y_zero_transform_bypass_flag
        if decoder.read_bool() { // seq_scaling_matrix_present_flag
            sps.scaling_list_count = if chroma_format_idc != 3 { 8 } else { 12 };
            for i in 0..sps.scaling_list_count {
                if decoder.read_bool() { // seq_scaling_list_present_flag[ i ]
                    if i < 6 {
                        // H264Parser.skipScalingList(decoder, 16);
                    } else {
                        // H264Parser.skipScalingList(decoder, 64);
                    }
                }
            }
        }
    }

    decoder.skip_ueg(); // log2_max_frame_num_minus4
    let pic_order_cnt_type = decoder.read_ueg();
    if pic_order_cnt_type == 0 {
        decoder.read_ueg(); // log2_max_pic_order_cnt_lsb_minus4
    } else if pic_order_cnt_type == 1 {
        decoder.skip_bits(1); // delta_pic_order_always_zero_flag
        decoder.skip_eg(); // offset_for_non_ref_pic
        decoder.skip_eg(); // offset_for_top_to_bottom_field
        sps.num_ref_frames_in_pic_order_cnt_cycle = decoder.read_ueg();
        
        for i in 0..sps.num_ref_frames_in_pic_order_cnt_cycle {
            decoder.skip_eg();
        }
    }

    decoder.skip_ueg(); // max_num_ref_frames
    decoder.skip_bits(1); // gaps_in_frame_num_value_allowed_flag
    sps.pic_width_in_mbs_minus1 = decoder.read_ueg();
    sps.pic_height_in_map_units_minus1 = decoder.read_ueg();
    sps.frame_mbs_only_flag = decoder.read_bits(1, true);
    
    if sps.frame_mbs_only_flag == 0 {
        decoder.skip_bits(1); // mb_adaptive_frame_field_flag
    }
    
    decoder.skip_bits(1); // direct_8x8_inference_flag
    
    if decoder.read_bool() { // frame_cropping_flag
        sps.frame_crop_left_offset = decoder.read_ueg();
        sps.frame_crop_right_offset = decoder.read_ueg();
        sps.frame_crop_top_offset = decoder.read_ueg();
        sps.frame_crop_bottom_offset = decoder.read_ueg();
    }

    if decoder.read_bool() {
        // vui_parameters_present_flag
        if decoder.read_bool() {
            // aspect_ratio_info_present_flag
            let aspect_ratio_idc = decoder.read_ubyte(1);
            let sar_ratio : Option<[u8; 2]>= match aspect_ratio_idc {
                1 => Some([1, 1]),
                2 => Some([12, 11]),
                3 => Some([10, 11]),
                4 => Some([16, 11]),
                5 => Some([40, 33]),
                6 => Some([24, 11]),
                7 => Some([20, 11]),
                8 => Some([32, 11]),
                9 => Some([80, 33]),
                10 => Some([18, 11]),
                11 => Some([15, 11]),
                12 => Some([64, 33]),
                13 => Some([160, 99]),
                14 => Some([4, 3]),
                15 => Some([3, 2]),
                16 => Some([2, 1]),
                255 => {
                    Some([  (decoder.read_ubyte(1) << 8 | decoder.read_ubyte(1)) as u8, 
                            (decoder.read_ubyte(1) << 8 | decoder.read_ubyte(1)) as u8])
                },
                _ => None
            };

            if sar_ratio.is_some() {
                let s = sar_ratio.unwrap();
                sps.sar_scale = s[0] / s[1];
            }
        }

        if decoder.read_bool() { decoder.skip_bits(1); }

        if decoder.read_bool() {
            decoder.skip_bits(4);
            if decoder.read_bool() {
                decoder.skip_bits(24);
            }
        }

        if decoder.read_bool() {
            decoder.skip_ueg();
            decoder.skip_ueg();
        }

        if decoder.read_bool() {
            sps.units_in_tick = decoder.read_uint();
            sps.time_scale = decoder.read_uint();
            sps.fixed_frame_rate = decoder.read_bool();
            sps.frame_duration = sps.time_scale / (2 * sps.units_in_tick);
        }
    }

    sps
}

struct Expo<'a> {
    pub data : &'a Vec<u8>,
    pub index : u32,
    pub bit_length : u32,
}

impl<'a> Expo<'a> {
    pub fn new(data: &'a Vec<u8>) -> Expo {
        Expo {
            index: 0,
            bit_length: data.len() as u32 * 8,
            data,
        }
    }

    pub fn bits_available(&self) -> u32 {
        self.bit_length - self.index
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

    pub fn get_bits(&mut self, size: u32, offset_bits: u32, move_index: bool) -> u32 {

        if self.bits_available() < size {
            println!("bits_available < size");
            return 0;
        }

        let offset = offset_bits % 8;
        let t = offset_bits / 8 | 0;
        let a : u8 = (0xff as u32 >> offset) as u8;
        let byte = self.data[t as usize] & a;
        let bits = 8 - offset;

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
            
            let a = self.get_bits(next_size, offset_bits + bits, move_index) as u8;
            let b: u8 = ((byte as u32) << next_size) as u8;
            (a | b) as u32
        }
    }

    pub fn skip_lz(&mut self) -> u32 {
        let leading_zero_count: u32 = 0;
        
        for leading_zero_count in 0..(self.bit_length - self.index) {

            if self.get_bits(1, self.index + leading_zero_count, false) > 0 {
                self.index = self.index + leading_zero_count;
                return leading_zero_count;
            }
        }

        return leading_zero_count;
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

#[derive(Debug, Clone, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum NalType {
    NDR = 1,
    IDR = 5,
    SEI = 6,
    SPS = 7,
    PPS = 8,
    AUD = 9,
}

#[derive(Clone, Debug)]
pub struct Nalu {
    pub payload : Vec<u8>,
    pub nri : u8,
    pub ntype: NalType,
}

impl Nalu {

    pub fn new(data: Vec<u8>) -> Nalu {
        Nalu {
            nri: (data[0] & 0x60) >> 5,
            ntype: NalType::try_from(data[0] & 0x1f).unwrap(),
            payload: data,
        }
    }

    pub fn is_keyframe(&self) -> bool {
        self.ntype == NalType::IDR
    }

    pub fn get_size(&self ) -> usize {
        return 4 + self.payload.len();
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

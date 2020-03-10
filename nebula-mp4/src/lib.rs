pub mod h264;
pub mod mp4;


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use super::*;

    fn load_file() -> Vec<u8> {
        let mut f = File::open("video.bin").expect("unable to load file");
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).expect("unable to read file");
        buffer
    }

    #[test]
    fn create_empty_track() {
        let track : mp4::Track = mp4::Track::new();
        assert_eq!(track.id, 0);
        assert_eq!(track.track_type, "video");
        // Track {
        //     id: 0,
        //     track_type: String::from("video"),
        //     duration: 1000,
        //     timescale: 1000,
        //     samples: vec![],
        //     pps: vec![],
        //     sps: vec![],
        //     config: vec![],
        //     width: 0,
        //     height: 0,
        //     audio_sample_rate: 0,
        //     codec: String::from(""),
        //     channel_count: 0,
        //     volume: 0,
        // }
    }
    #[test]
    fn parse_nalus() {
        let data = load_file();
        let data_size = data.len();
        assert!(data_size > 0);
        let nalus = h264::parse_to_nalu(data);
        assert!(nalus.len() > 0);
        let mut size = 0;
        let mut has_sps = false;
        let mut has_pps = false;
        let mut has_keyframe = false;
        for nalu in nalus {
            size = size + nalu.get_size();
            match nalu.ntype {
                h264::NalType::SPS => has_sps = true,
                h264::NalType::PPS => has_pps = true,
                h264::NalType::IDR | h264::NalType::NDR => has_keyframe = true,
                _ => (),
            }
        }
        assert!(has_sps);
        assert!(has_pps);
        assert!(has_keyframe);
        assert_eq!(size, data_size);
    }

    #[test]
    fn parse_sps() {
        let data = load_file();
        let nalus = h264::parse_to_nalu(data);
        let mut video_track = mp4::Track::new();
        for nalu in nalus {
            match nalu.ntype {

                h264::NalType::SPS => {
                    video_track.parse_sps(nalu.payload);
                },
                _ => (),
            }
        }
    }



}

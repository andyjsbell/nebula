extern crate nebula_mp4;

use nebula_mp4::{
    h264,
    mp4
};

use tungstenite::{connect, Message};
use url::Url;
use std::time::Instant;
fn main() {

    let (mut socket, _) =
        connect(Url::parse("ws://localhost:9001/socket").unwrap()).expect("Can't connect");
    
    let mut now = Instant::now();
    let mut count = 0;
    let mut initialised: bool = false;
    loop {
        // println!("request frame");
        socket
            .write_message(Message::Binary(vec!['f' as u8]))
            .unwrap();
        
        let message = socket.read_message().expect("Error reading message");
        // println!("frame received");
        count = count + 1;
        if count == 100 {
            let frame_ms = now.elapsed().as_millis() / 100;
            println!("fps:{}", 1000 / frame_ms);
            now = Instant::now();
            count = 0;
        } 

        let nalus = h264::parse_to_nalu(message.into_data());
            let mut size = 0;
            println!("nalus size: {}", nalus.len());
            let mut video_track = mp4::Track::new();
            let mut units : Vec<h264::Nalu> = Vec::new();
            for nalu in nalus {
                size = size + nalu.get_size();
                match nalu.ntype {

                    h264::NalType::SPS => {
                        println!("processing SPS");
                        units.push(nalu.clone());
                        video_track.parse_sps(nalu.payload);
                    },
                    h264::NalType::PPS => {
                        println!("processing PPS");
                        units.push(nalu.clone());
                        video_track.parse_pps(nalu.payload);
                    },
                    h264::NalType::IDR | h264::NalType::NDR => {
                        println!("processing IDR or NDR");
                        units.push(nalu.clone());
                        
                        video_track.samples.push(mp4::Sample {
                            cts: 0,
                            duration: 30,  // We are assuming 30ish frames per second, TODO - need to add to protocol duration
                            flags: mp4::Flags {
                                is_leading: 0,
                                is_depended_on: 0,
                                has_redundancy: 0,
                                degrad_prio: 0,
                                is_non_sync: 0,
                                depends_on: 0,
                                padding_value: 0,
                            },
                            nalus: vec![nalu],
                            size,
                        });
                    },
                    _ => {
                        ();
                    }
                }
            }
            
            if !initialised {
                let mime = format!("video/mp4; codecs=\"{}\"", video_track.codec);
                println!("mime = {}", mime);
                initialised = true;
                // let source_buffer = media_source.add_source_buffer(&mime).unwrap();
                // source_buffer.append_buffer_with_array_buffer(&a.buffer()).unwrap();
                // mp4::init_segment(vec![video_track], 0xffffffff, 1000);
            } else {
                let sequence_number = 0; // this needs to increase on each atom
                let decode_time = 0;
                let mut moof = mp4::moof(sequence_number, decode_time, &video_track);
                let mut mdat = mp4::mdat([0,0,0,0]);
                moof.append(&mut mdat);
            }
    }
}

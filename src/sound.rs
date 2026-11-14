use std::{fs::File, io::BufReader};
use rodio::{MixerDeviceSink};

#[derive(Debug)]
pub struct Sound {
    sink_handle: MixerDeviceSink,
}

impl Sound {
    pub fn new() -> Self {

        let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
        .expect("open default audio stream");

        Self {
            sink_handle
        }
    }

    pub fn play_beep(&self) {
        let beep_file = BufReader::new(File::open("assets/beep.mp3").unwrap());
        let player = rodio::play(&self.sink_handle.mixer(), beep_file).unwrap();
        player.detach();
    }
}
use std::{thread,time,io::BufReader};
use std::fs::File;
use cpal::DefaultStreamConfigError;
use rodio::{Decoder, OutputStream, source::Source};

pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

pub fn play_mp3(filename: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(format!("../sound/{filename}")).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
}

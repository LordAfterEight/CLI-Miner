use std::{thread,time};

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	Tween,
};


pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

pub fn play_mp3(soundname: String) {
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default());
    let mut sound_data = StaticSoundData::from_file("../sound/{soundname}");
    let mut sound = manager.expect("REASON").play(sound_data);
}

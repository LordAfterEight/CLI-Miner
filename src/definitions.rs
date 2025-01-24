use std::{thread,time};

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
	sound::{SoundData,FromFileError,static_sound::{StaticSoundData, StaticSoundSettings, StaticSoundHandle}},
	Tween,
    PlaySoundError
};


pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

pub fn load_file(soundname: &str) -> Result<(), FromFileError>{
    let sound_data = StaticSoundData::from_file(format!("../sound/{soundname}"))?;
    Ok(())
}


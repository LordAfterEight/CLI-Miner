use std::{thread,time};
pub fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time));
}

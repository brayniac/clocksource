extern crate clocksource;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

fn main() {
	let clock = clocksource::Clocksource::new();
	for _ in 0..60 {
		let cs_time = clock.time();
		let sys_time = SystemTime::now();
		let sys_dur = sys_time.duration_since(UNIX_EPOCH).expect("failure getting system time");
		let sys_time = sys_dur.as_secs() * 1_000_000_000 + sys_dur.subsec_nanos() as u64;

		let delta = cs_time as i64 - sys_time as i64;

		println!("Clocksource: {} System: {} Delta: {}", cs_time, sys_time, delta);
		thread::sleep(Duration::new(1, 0));

	}

}
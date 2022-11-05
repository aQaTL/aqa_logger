use std::sync::Once;

pub use log::*;

pub struct Logger;

static LOGGER: Logger = Logger;
static LOGGER_SET_LOCK: Once = Once::new();

pub fn init() {
	let level = if cfg!(debug_assertions) {
		LevelFilter::Debug
	} else {
		LevelFilter::Info
	};
	init_with_level(level)
}

pub fn init_with_level(level: LevelFilter) {
	LOGGER_SET_LOCK.call_once(|| {
		set_logger(&LOGGER).expect("Tried to set global logger twice");
		set_max_level(level);
	});
}

static COLORS: [u32; 5] = [91, 31, 33, 36, 90];

impl Log for Logger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		println!(
			"\x1B[0;{}m[{}][{}]\x1B[0m {}",
			COLORS[record.level() as usize - 1],
			record.level(),
			record.metadata().target(),
			record.args(),
		);
	}

	fn flush(&self) {}
}

#[test]
fn color_test() {
	for c in COLORS {
		println!("\x1B[0;{}mHello World\x1b[0m", c);
	}
}

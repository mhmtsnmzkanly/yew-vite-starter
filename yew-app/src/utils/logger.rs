use std::cell::RefCell;
use log::{info, Level};
use wasm_logger::{init, Config};

// Logger is ready?
thread_local!(static LOGGER_INITED: RefCell<bool> = RefCell::new(false));

pub fn _init() {
	LOGGER_INITED.with(|_bool| {
		if _bool.clone().into_inner() {
			info!("Logger is already inited.");
			return;
		}
		
		*_bool.borrow_mut() = true;
		init(Config::new(Level::Info))
	})
}
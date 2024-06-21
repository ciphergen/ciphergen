use std::panic::set_hook;

use log::error;

pub fn setup_panic() {
    set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|location| (location.file(), location.line(), location.column()));

        if let Some((file, line, column)) = location {
            error!("Panic occurred in {file} at {line}:{column}");
        }

        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            error!("{payload}");
        }

        if let Some(payload) = info.payload().downcast_ref::<String>() {
            error!("{payload}");
        }
    }));
}

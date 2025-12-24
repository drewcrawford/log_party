#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use web_sys::console;


/// Logs messages to console at each level
#[wasm_bindgen]
pub fn do_log() {
    console::debug_1(&"debug: log_party".into());
    console::info_1(&"info: log_party".into());
    console::log_1(&"log: log_party".into());
    console::warn_1(&"warn: log_party".into());
    console::error_1(&"error: log_party".into());
}

/// Panics
#[wasm_bindgen]
pub fn do_panic() {
    panic!("log_party panic!");
}

/// Panics with console_error_panic_hook enabled
#[wasm_bindgen]
pub fn do_panic_hook() {
    console_error_panic_hook::set_once();
    panic!("log_party panic!");
}
//
// /// Spawns a thread and logs from within it
// #[wasm_bindgen]
// pub fn thread_log() {
//     wasm_safe_thread::spawn(|| {
//         console::debug_1(&"debug: log_party (from thread)".into());
//         console::info_1(&"info: log_party (from thread)".into());
//         console::log_1(&"log: log_party (from thread)".into());
//         console::warn_1(&"warn: log_party (from thread)".into());
//         console::error_1(&"error: log_party (from thread)".into());
//     });
// }
//
// /// Spawns a thread and panics from within it
// #[wasm_bindgen]
// pub fn thread_panic() {
//     wasm_safe_thread::spawn(|| {
//         panic!("log_party panic from thread!");
//     });
// }

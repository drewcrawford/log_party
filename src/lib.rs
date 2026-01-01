#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use web_sys::console;


/// Logs messages to console at each level
#[wasm_bindgen]
pub fn do_log() {
    console::debug_1(&"MAIN_DEBUG_2c7f9a".into());
    console::info_1(&"MAIN_INFO_4e8b3d".into());
    console::log_1(&"MAIN_LOG_6a1c5e".into());
    console::warn_1(&"MAIN_WARN_8d4f2b".into());
    console::error_1(&"MAIN_ERROR_0f3e7c".into());
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

/// Spawns a thread and logs from within it
#[wasm_bindgen]
pub async fn thread_log() {
    let _ = wasm_safe_thread::spawn(|| {
        console::debug_1(&"THREAD_DEBUG_8f3a2b".into());
        console::info_1(&"THREAD_INFO_9c4d1e".into());
        console::log_1(&"THREAD_LOG_7b2e5f".into());
        console::warn_1(&"THREAD_WARN_3a6c8d".into());
        console::error_1(&"THREAD_ERROR_1d9f4a".into());
    }).join_async().await;

}

/// Spawns a thread and panics from within it
#[wasm_bindgen]
pub async fn thread_panic() {
    wasm_safe_thread::spawn(|| {
        panic!("log_party panic from thread!");
    }).join_async().await.unwrap();
}

/// Spawns a thread and panics from within it with console_error_panic_hook enabled
#[wasm_bindgen]
pub async fn thread_panic_hook() {
    wasm_safe_thread::spawn(|| {
        console_error_panic_hook::set_once();
        panic!("log_party panic from thread!");
    }).join_async().await.unwrap();
}

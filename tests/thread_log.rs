#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn test_thread_log() {
    log_party::thread_log().await;
}

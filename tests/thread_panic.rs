#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

/// This test spawns a thread that panics.
/// We expect to see "log_party panic from thread!" in the output.
/// Note: #[should_panic] doesn't work for thread panics in wasm-bindgen-test-runner.
#[wasm_bindgen_test]
async fn test_thread_panic() {
    log_party::thread_panic().await;
}

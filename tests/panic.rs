#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_panic() {
    log_party::do_panic();
}

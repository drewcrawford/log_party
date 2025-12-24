#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
#[should_panic(expected = "log_party panic!")]
fn test_panic() {
    log_party::do_panic();
}

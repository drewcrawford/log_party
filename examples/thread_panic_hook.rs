#![cfg(target_arch = "wasm32")]

fn main() {
    wasm_bindgen_futures::spawn_local(log_party::thread_panic_hook());
}

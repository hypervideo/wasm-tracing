use tracing::{event, span, Level};
use wasm_bindgen_test::*;
use wasm_tracing::WasmLayerConfig;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn simple_test() {
    console_error_panic_hook::set_once();
    let config = WasmLayerConfig::new()
        .set_max_level(Level::DEBUG)
        .set_show_origin(false)
        .set_show_fields(false)
        .to_owned();
    wasm_tracing::set_as_global_default_with_config(config).unwrap();

    throw_events();
}

pub fn throw_events() {
    event!(Level::INFO, "Foobar");
    event!(Level::WARN, "Warn log");
    let span = span!(Level::INFO, "Test span");
    let _guard = span.enter();
    event!(Level::DEBUG, "Inside span");
    event!(Level::ERROR, "Error log");
}

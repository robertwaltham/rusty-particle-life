use wasm_bindgen::prelude::*;
use web_sys::console;

fn main() {
    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);

    let document = web_sys::window().unwrap().document().unwrap();

    let loc = document.location().unwrap();

    console::log_1(&loc.into());
}

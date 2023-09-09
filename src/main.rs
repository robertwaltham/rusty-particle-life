use bevy::prelude::*;
use js_sys::Object;
use web_sys::console;

fn main() {
    // let settings = WgpuSettings::default();
    // let backends = settings.backends.unwrap();
    // println!("{:?}", backends);

    // if backends & Backends::BROWSER_WEBGPU == Backends::BROWSER_WEBGPU {
    //     console::log_1(&"supports webgpu".into());
    // } else {
    //     console::log_1(&"does not support webgpu".into());
    // }

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // let loc = document.location().unwrap();
    // console::log_1(&loc.into());

    // https://developer.mozilla.org/en-US/docs/Web/API/GPU
    // Firefox has navigator.gpu, despite the docs saying it's nightly only
    let gpu = web_sys::window().unwrap().navigator().gpu();
    if gpu.is_undefined()
        || !Object::get_prototype_of(&gpu).has_own_property(&"wgslLanguageFeatures".into())
    {
        let error_msg = "Sorry, WebGPU is not supported on this browser. Only Chrome 113 and later is supported.";
        console::log_1(&error_msg.into());

        // Manufacture the element we're gonna append
        let val = document.create_element("p").expect("no element");
        val.set_text_content(Some(error_msg));

        body.append_child(&val).expect("couldn't add node");
    } else {
        App::new().add_plugins(DefaultPlugins).run();
    }
}

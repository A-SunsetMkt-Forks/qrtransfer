mod encoder;
mod utils;
use utils::{log, set_panic_hook};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send(file_name: &str, int_array: &JsValue) -> Result<(), JsValue> {
    set_panic_hook();
    let html = {
        log(file_name);
        let int_array: Vec<u8> = int_array.into_serde().unwrap();
        log(&format!("{:?}", int_array));

        encoder::Encoder::new(file_name.to_string(), int_array).to_html()
    };

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let qrcode_div = document
        .get_element_by_id("qrcode")
        .expect("should have a qrcode element");

    qrcode_div.set_inner_html(&html);

    Ok(())
}

#[wasm_bindgen]
pub fn receive() -> Result<(), JsValue> {
    set_panic_hook();

    let chunks = vec![
        "77u/VHJhbnNmZXIgeW91ciBmaWxlIGZyb20gYW4gYWlyIGdhcHBlZCBjb21wdXRlciB0byBpT1MvaVBob25lL2lQYWQgdXNpbmcgb25seSBxcmNvZGUsIG5vIHdpZmkvdXNiLw==" ,
        "Ymx1ZXRvb3RoIG5lZWRlZC4="
        ];

    let data = chunks
        .iter()
        .map(|s| base64::decode(s).unwrap())
        .collect::<Vec<Vec<u8>>>()
        .concat();

    let base64_data = base64::encode(data);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let a = document.create_element("a")?;
    a.set_attribute("href", &("data:;base64,".to_string() + &base64_data))
        .unwrap();
    a.set_attribute("download", "binFile.txt").unwrap();
    a.set_inner_html("Download");

    document
        .body()
        .expect("no body")
        .append_child(&a)
        .expect("failed to append");

    Ok(())
}

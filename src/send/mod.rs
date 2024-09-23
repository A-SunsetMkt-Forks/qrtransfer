#![allow(non_snake_case)]

pub mod encoder;
mod scroll;

use crate::utils::log;
use crate::QR_RES;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dioxus::prelude::*;
use indexmap::IndexMap;

pub use scroll::toggle_scroll;

#[derive(PartialEq, Clone, Props, Default)]
pub struct QrRes {
    pub payloads: IndexMap<String, String>,
}

pub fn QrResPage(props: QrRes) -> Element {
    rsx! {
        {
            props.payloads.iter().map(|(name, svg)| {
                rsx! {
                    table {
                        style: "float:left;",
                        tr {td {class: "qr", dangerous_inner_html: "{svg}"}}
                        tr {td {"align": "center", "{name}"}}
                    }
                }
            })
        }
    }
}

fn send(file_name: String, data: Vec<u8>) {
    log(&format!("Sending file: {}", file_name));
    let qr = encoder::Encoder::new(file_name, data).to_qr();
    log("setting QR_RES");

    *QR_RES.write() = qr;
    log("QR_RES set");
}

pub async fn read_file_content() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let progress_div = document
        .get_element_by_id("progress")
        .expect("should have a progress_div element");
    progress_div.set_inner_html("Processing...");

    let filelist = document
        .get_element_by_id("file-selector")
        .expect("should have a file-selector element")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .files()
        .expect("Failed to get filelist from File Input!");

    let file = filelist.get(0).expect("Failed to get File from filelist!");
    let file_name = file.name();
    log(&file_name);

    let file_reader = web_sys::FileReader::new().unwrap();

    let fr_c = file_reader.clone();

    let (rx, tx) = futures::channel::oneshot::channel();
    let onloadend_cb: Closure<dyn FnMut()> = Closure::new({
        let mut rx = Some(rx);
        move || {
            let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
            let _ = rx
                .take()
                .expect("multiple files read without refreshing the channel")
                .send(array.to_vec());
        }
    });

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    onloadend_cb.forget();
    file_reader
        .read_as_array_buffer(&file)
        .expect("blob not readable");

    let array = tx.await.unwrap();
    send(file_name.clone(), array);
}

pub mod compress;
pub mod decoder;
pub mod send;
pub mod utils;

#[test]
fn test_integration() {
    let file_name = "test_qrtransfer.txt";
    let file_content = "Transfer your file from an air gapped computer to iOS/iPhone/iPad using only qrcode, no wifi/usb/bluetooth needed. This is a proof-of-concept project, implemented in Rust WebAssembly.";

    let int_array = Vec::from(file_content.as_bytes());
    let int_array = compress::compress(int_array);

    let encoder = send::encoder::Encoder::new(file_name.to_string(), int_array);

    let mut decoder = decoder::Decoder::new();
    for (_name, payload) in encoder.payloads() {
        decoder.process_chunk(payload);
    }

    let decoded_data = base64::decode(decoder.to_base64()).unwrap();
    let decoded_data = String::from_utf8(decoded_data).unwrap();

    assert_eq!(file_name, decoder.get_name());
    assert_eq!(file_content, decoded_data);
}

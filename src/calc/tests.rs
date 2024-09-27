use opencv::imgcodecs::imread_def;
use rxing::helpers::detect_multiple_in_file;
use super::*;

#[test]
fn test_calc() {
    let detector = QRCodeDetector::default().unwrap();
    let mut decoded_info = Vector::new();
    let img = imread_def("qrcode2.jpg").unwrap();
    let result = detector.detect_and_decode_multi_def(&img, &mut decoded_info);
    assert!(result.is_ok());
    println!("Decoded info: {:?}", decoded_info);
}

#[test]
fn test_calc_rxing() {
    let results = detect_multiple_in_file("qrcode2.jpg").unwrap();
    for result in results {
        println!("{} -> {}", result.getBarcodeFormat(), result.getText())
    }
}
use opencv::imgcodecs::imread_def;
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
fn test_calc_wechat() {
    let mut detector = WeChatQRCode::new(
        "./model/detect.prototxt",
        "./model/detect.caffemodel",
        "./model/sr.prototxt",
        "./model/sr.caffemodel",
    ).unwrap();
    let img = imread_def("qrcode2.jpg").unwrap();
    let results = detector.detect_and_decode_def(&img);
    assert!(results.is_ok());
    println!("Decoded info: {:?}", results);
}
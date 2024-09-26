use super::*;


#[test]
fn test_vd_picker() {
    let vd_picker = VDevice::vd_picker().unwrap();
    println!("Selected device: {}", vd_picker.index);
}
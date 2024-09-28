use crossbeam_channel::{select, Receiver, Sender};
use opencv::core::Vector;
use opencv::prelude::*;
use opencv::{wechat_qrcode::WeChatQRCode};
use std::thread;
use opencv::objdetect::QRCodeDetector;

#[cfg(test)]
mod tests;

pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Self {}
    }

    fn calc_diff(v: &Vector<String>) -> Option<usize> {
        if v.len() < 2 {
            return None;
        }
        let mut timestamps: Vec<usize> = Vec::new();
        for item in v {
            if let Ok(value) = item.parse::<usize>() {
                timestamps.push(value);
            }
        }
        if timestamps.len() < 2 {
            return None;
        }
        let max = timestamps.iter().max()?;
        let min = timestamps.iter().min()?;

        Some(max - min)
    }

    pub fn calc_one(m: &Mat) -> Option<String> {
        let detector = QRCodeDetector::default().unwrap();
        let result = detector.detect_and_decode_def(&m);
        match result {
            Ok(decoded_info) => {
                decoded_info.iter().map(|x| x.to_string()).collect::<String>().into()
            }
            Err(_) => {
                None
            }
        }
    }


    pub fn is_multi_qr(m: &Mat) -> (bool, usize) {
        let mut detector = WeChatQRCode::new(
            "./model/detect.prototxt",
            "./model/detect.caffemodel",
            "./model/sr.prototxt",
            "./model/sr.caffemodel",
        ).unwrap();
        let result = detector.detect_and_decode_def(&m);
        match result {
            Ok(ref decoded_info) => {
                let info = Self::calc_diff(decoded_info);
                match info {
                    Some(diff) => {
                        (true, diff)
                    }
                    None => {
                        (false, 0)
                    }
                }
            }
            Err(_) => {
                (false, 0)
            }
        }
    }

    pub fn worker(self, tx: Sender<String>, rx: Receiver<Mat>) {
        let mut detector = WeChatQRCode::new(
            "./model/detect.prototxt",
            "./model/detect.caffemodel",
            "./model/sr.prototxt",
            "./model/sr.caffemodel",
        ).unwrap();
        let rx_clone = rx.clone();
        let tx_clone = tx.clone();
        #[cfg(debug_assertions)]
        let mut latest_detected = std::time::Instant::now();
        thread::spawn(move || {
            loop {
                select! {
                recv(rx_clone) -> mat => {
                    match mat {
                        Ok(mat) => {
                            let result = detector.detect_and_decode_def(&mat);
                            match result {
                                Ok(ref decoded_info) => {
                                    let info = Self::calc_diff(decoded_info);
                                    match info {
                                        Some(diff) => {
                                            #[cfg(debug_assertions)]
                                            {
                                                let now = std::time::Instant::now();
                                                let detected_time = now.duration_since(latest_detected);
                                                latest_detected = now;
                                                println!("Elapsed: {:?}", detected_time);
                                            }
                                            println!("Delay: {}", diff);
                                            tx_clone.send(diff.to_string()).unwrap();
                                        }
                                        None => {
                                            eprintln!("[Calc] Error: Cannot calculate diff");
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[Calc] Error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[Calc] Error: {}", e);
                        }
                    }
                }
            }
            }
        });
    }
}
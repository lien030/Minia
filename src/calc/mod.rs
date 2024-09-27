use std::thread;
use crossbeam_channel::{select, Receiver, Sender};
use opencv::core::Vector;
use opencv::objdetect::QRCodeDetector;
use opencv::prelude::*;

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

    pub fn detect_rxing_multi(m: &Mat) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let height = m.rows();
        let width = m.cols();
        // mat -> luma
        let mut gray = Mat::default();
        opencv::imgproc::cvt_color(&m, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0)?;

        let luma_data = gray.data_bytes()?.to_vec();

        let results = rxing::helpers::detect_multiple_in_luma(luma_data, width as u32, height as u32)?;
        let mut decoded_info = Vec::new();
        for result in results {
            decoded_info.push(result.getText().to_string());
        }
        Ok(decoded_info)
    }

    pub fn is_multi_qr(m: &Mat) -> (bool, usize) {
        let detector = QRCodeDetector::default().unwrap();
        let mut decoded_info = Vector::new();
        let result = detector.detect_and_decode_multi_def(&m, &mut decoded_info);
        println!("Decoded info: {:?}", decoded_info);
        if result.is_ok() {
            if let Some(diff) = Calc::calc_diff(&decoded_info) {
                (true, diff)
            } else {
                (false, 0)
            }
        } else {
            (false, 0)
        }
    }

    pub fn is_multi_qr_rxing(m: &Mat) -> (bool, usize) {
        let results = Calc::detect_rxing_multi(&m);
        // Vec<String> -> Vector<String>
        match results {
            Ok(decoded_info) => {
                if decoded_info.len() > 1 {
                    let vectors = Vector::from(decoded_info.iter().map(|x| x.as_str()).collect::<Vec<&str>>());
                    if let Some(diff) = Calc::calc_diff(&vectors) {
                        (true, diff)
                    } else {
                        (false, 0)
                    }
                } else {
                    (false, 0)
                }
            }
            Err(_) => {
                (false, 0)
            }
        }
    }

    pub fn worker(self, tx: Sender<String>, rx: Receiver<Mat>) {
        let detector = QRCodeDetector::default().unwrap();
        let rx_clone = rx.clone();
        let tx_clone = tx.clone();
        thread::spawn(move || {
            loop {
                select! {
                recv(rx_clone) -> mat => {
                    match mat {
                        Ok(mat) => {
                            let mut decoded_info = Vector::new();
                            let result = detector.detect_and_decode_multi_def(&mat, &mut decoded_info);
                            if result.is_ok() {
                                if let Some(diff) = Calc::calc_diff(&decoded_info) {
                                    match tx_clone.send(diff.to_string()) {
                                        Ok(_) => {
                                                println!("Diff: {}ms", diff);
                                            }
                                        Err(e) => {
                                            eprintln!("[Calc] Error: {}", e);
                                        }
                                    }
                                } else {
                                    println!("Cannot calculate diff");
                                }
                            } else {
                                println!("Cannot decode");
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

    pub fn worker_rxing(self, tx: Sender<String>, rx: Receiver<Mat>) {
        let rx_clone = rx.clone();
        let tx_clone = tx.clone();
        thread::spawn(move || {
            loop {
                select! {
                recv(rx_clone) -> mat => {
                    match mat {
                        Ok(mat) => {
                            let results = Calc::detect_rxing_multi(&mat);
                            match results {
                                Ok(decoded_info) => {
                                    let diff = Calc::calc_diff(&Vector::from(decoded_info.iter().map(|x| x.as_str()).collect::<Vec<&str>>()));
                                    match diff {
                                        Some(diff) => {
                                            match tx_clone.send(diff.to_string()) {
                                                Ok(_) => {
                                                    println!("Diff: {}ms", diff);
                                                }
                                                Err(e) => {
                                                    eprintln!("[Calc] Error: {}", e);
                                                }
                                            }
                                        }
                                        None => {
                                            println!("Cannot calculate diff");
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
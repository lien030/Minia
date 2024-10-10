use std::error::Error;
use std::{io, thread};
use std::io::{ErrorKind};
use crossbeam_channel::Sender;
use opencv::{highgui, videoio};
use opencv::core::{Point, Rect, Scalar};
use opencv::highgui::destroy_window;
use opencv::imgproc::{put_text, rectangle, FONT_HERSHEY_DUPLEX};
use opencv::prelude::*;

#[cfg(test)]
mod tests;

static TIPS: &str = "Press n: next, p: previous, c: continue, q: quit";

pub struct VDevice {
    index: i32,
}

impl VDevice {
    pub fn new(index: i32) -> Self {
        Self {
            index
        }
    }
    pub fn vd_picker() -> Result<Self, Box<dyn Error>> {
        let window_title = "Select a device - Minia";
        highgui::named_window(window_title, highgui::WINDOW_AUTOSIZE)?;
        let mut index = 0;
        loop {
            let cam = videoio::VideoCapture::new(index, videoio::CAP_DSHOW);
            match cam {
                Ok(mut cam) => {
                    if videoio::VideoCapture::is_opened(&cam)? {
                        // TODO:
                        loop {
                            let mut frame = Mat::default();
                            cam.read(&mut frame)?;
                            let height = frame.size()?.height;
                            if !frame.empty() {
                                // index
                                put_text(&mut frame, &format!("Device: {}", index), Point::from((10, 30)), FONT_HERSHEY_DUPLEX, 0.6, Scalar::from((232.0, 122.0, 114.0)), 1, 0, false)?;
                                // tips
                                put_text(&mut frame, TIPS, Point::from((10, height - 20)), FONT_HERSHEY_DUPLEX, 0.6, Scalar::from((232.0, 122.0, 114.0)), 1, 0, false)?;
                                highgui::imshow(&window_title, &frame)?;
                            }
                            let key = highgui::wait_key(10)?;
                            match key {
                                78 | 110 => {
                                    // n or N: next device
                                    index += 1;
                                    break;
                                }
                                80 | 112 => {
                                    // p or P: previous device
                                    if index - 1 >= 0 {
                                        index -= 1;
                                        break;
                                    }
                                    println!("No previous device");
                                }
                                67 | 99 => {
                                    // c or C: continue
                                    destroy_window(window_title)?;
                                    return Ok(Self::new(index));
                                }
                                81 | 113 => {
                                    // q or Q: quit
                                    return Err(io::Error::new(ErrorKind::Interrupted, "User quit").into());
                                }
                                _ => {}
                            }
                        }
                    } else {
                        println!("Device {} is not available", index);
                        index = 0;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    index = 0;
                }
            }
        }
    }

    pub fn check_multi_qrcodes(&self) -> Result<(), Box<dyn Error>> {
        let titile = "Check multi QR codes - Minia";
        let mut frame = Mat::default();
        let mut cam = videoio::VideoCapture::new(self.index, videoio::CAP_DSHOW)?;
        highgui::named_window(titile, highgui::WINDOW_AUTOSIZE)?;
        loop {
            cam.read(&mut frame).unwrap();

            let height = frame.size()?.height;
            let width = frame.size()?.width;

            let (is_multi, diff) = super::calc::Calc::is_multi_qr(&frame);

            // blue rectangle
            let mut rect_color = Scalar::from((255.0, 0.0, 0.0));
            let mut tips = String::from("No QR code, Press q to quit");
            if is_multi {
                // green rectangle
                rect_color = Scalar::from((0.0, 255.0, 0.0));
                tips = String::from(format!("Delay: {}, Press c to continue", diff));
            }

            rectangle(&mut frame, Rect::new(0, 0, width, height), rect_color, 2, 8, 0)?;
            put_text(&mut frame, &tips, Point::from((10, 30)), FONT_HERSHEY_DUPLEX, 0.6, Scalar::from((232.0, 122.0, 114.0)), 1, 0, false)?;
            highgui::imshow(&titile, &frame)?;

            let key = highgui::wait_key(10)?;
            match key {
                81 | 113 => {
                    // q or Q: quit
                    return Err(io::Error::new(ErrorKind::Interrupted, "User quit").into());
                }
                67 | 99 => {
                    // c or C: continue
                    if is_multi {
                        destroy_window(titile)?;
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn worker(&self, tx: Sender<Mat>, fps: u64) -> Result<(), Box<dyn Error>> {
        let tx_clone = tx.clone();
        let mut cam = videoio::VideoCapture::new(self.index, videoio::CAP_DSHOW)?;
        loop {
            let mut frame = Mat::default();
            cam.read(&mut frame)?;
            if !frame.empty() {
                tx_clone.send(frame)?;
            }
            thread::sleep(std::time::Duration::from_millis(1000 / fps));
        }
    }
}
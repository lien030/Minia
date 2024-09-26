use std::error::Error;
use std::io;
use std::io::{ErrorKind};
use opencv::{highgui, videoio};
use opencv::core::{Point, Scalar};
use opencv::imgproc::{put_text, FONT_HERSHEY_DUPLEX};
use opencv::prelude::*;

#[cfg(test)]
mod tests;

static TIPS: &str = "Press n: next, p: previous, y: yes, q: quit";

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
            let cam = videoio::VideoCapture::new(index, videoio::CAP_ANY);
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
                                highgui::imshow(window_title, &frame)?;
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
                                89 | 121 => {
                                    // y or Y: yes
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
}
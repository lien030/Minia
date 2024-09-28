use crossbeam_channel::{Receiver, Sender, unbounded, bounded};
use once_cell::sync::Lazy;
use opencv::core::Mat;

mod csv_operator;
mod capture;
mod calc;

// opencv -(chan)-> calc -(chan)-> csv
// 2 unbounded channel: for Img(Mat) and for Delay(String)

static MAT_BUFFER_SIZE: usize = 1;
static MAT_CHANNEL: Lazy<(Sender<Mat>, Receiver<Mat>)> = Lazy::new(|| bounded(MAT_BUFFER_SIZE));
static DELAY_CHANNEL: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| unbounded());

static CAP_FPS: u64 = 30;

fn main() {
    let capture = capture::VDevice::vd_picker().unwrap();
    capture.check_multi_qrcodes().unwrap();

    let csv_op = csv_operator::CsvOperator::new(true).unwrap();
    let (ref tx_delay, ref rx_delay) = *DELAY_CHANNEL;
    csv_op.worker(rx_delay.clone());

    let (ref tx_mat, ref rx_mat) = *MAT_CHANNEL;
    let img_calc = calc::Calc::new();
    // img_calc.worker(tx_delay.clone(), rx_mat.clone());
    img_calc.worker(tx_delay.clone(), rx_mat.clone());

    println!("Started at: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    capture.worker(tx_mat.clone(), CAP_FPS).unwrap();
}
use crossbeam_channel::{Receiver, Sender, unbounded};
use once_cell::sync::Lazy;
use opencv::core::Mat;

mod csv_operator;
mod capture;

// opencv -(chan)-> calc -(chan)-> csv
// 2 unbounded channel: for Img(Mat) and for Delay(String)

static MAT_CHANNEL: Lazy<(Sender<Mat>, Receiver<Mat>)> = Lazy::new(|| unbounded());
static DELAY_CHANNEL: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| unbounded());

fn main() {
    println!("Started at: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    // println!("{:?}",opencv::core::get_cuda_enabled_device_count())
    // let csv_op = csv_operator::CsvOperator::new(true).unwrap();
    // let (ref tx, ref rx) = *DELAY_CHANNEL;
    // csv_op.worker(rx.clone());
}
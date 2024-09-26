use std::error::Error;
use std::fs::{File, OpenOptions};
use std::{io, thread};
use chrono::Local;
use crossbeam_channel::{select, Receiver};

#[cfg(test)]
mod tests;

pub struct CsvOperator {
    pub file_name: String,
    timestamp: bool,
    file: File,
}

impl CsvOperator {
    pub fn new(timestamp: bool) -> Result<Self, io::Error> {
        // Minia_yyyy-mm-dd_hh-mm-ss.csv_operator
        let name = format!("Minia_{}.csv", Local::now().format("%Y-%m-%d_%H-%M-%S"));
        File::create(&name)?;
        println!("Created file: {}", name);
        let file = OpenOptions::new().write(true).append(true).open(&name)?;
        Ok(Self {
            file_name: name,
            timestamp,
            file,
        })
    }

    pub fn write_vec(&self, data: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(&self.file);
        for v in data {
            if self.timestamp {
                wtr.write_record(vec![format!("{}", Local::now().timestamp_millis()), v.to_string()])?;
            } else {
                wtr.write_record(&[v])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    // only this thread will use "self"
    pub fn worker(self, rx: Receiver<String>) {
        thread::spawn(move || {
            loop {
                select! {
                    recv(rx) -> msg => {
                        match msg {
                            Ok(msg) => {
                                let _ = self.write_vec(vec![msg.as_str()]);
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    }
                }
            }
        });
    }
}
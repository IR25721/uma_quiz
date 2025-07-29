use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::read_file::ReadFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct CorrectRateRecord {
    total_answered: usize,
    total_corrected: usize,
}

impl ReadFile for CorrectRateRecord {
    fn new<T: AsRef<Path>>(data_path: T) -> Result<Self> {
        let json_content = fs::read_to_string(data_path)?;
        let record: CorrectRateRecord = serde_json::from_str(&json_content)?;
        Ok(record)
    }
}

impl CorrectRateRecord {
    pub fn add_answered(&mut self, answered: usize) {
        self.total_answered += answered;
    }

    pub fn add_corrected(&mut self, corrected: usize) {
        self.total_corrected += corrected;
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn print_stats(&self) {
        println!(
            "解答数: {} 回, 正解数: {} 回（正答率: {:.1}％）",
            self.total_answered,
            self.total_corrected,
            if self.total_answered > 0 {
                100.0 * self.total_corrected as f64 / self.total_answered as f64
            } else {
                0.0
            }
        );
    }
}

use chrono::{DateTime, Local};

use crate::read_file::ReadFile;
use color_eyre::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizHistoryRecord {
    quiz_titles: Vec<String>,
    quiz_timestamps: Vec<DateTime<Local>>,
}

impl ReadFile for QuizHistoryRecord {
    fn new<T: AsRef<Path>>(data_path: T) -> Result<Self> {
        let json_content = fs::read_to_string(data_path)?;
        let record: QuizHistoryRecord = serde_json::from_str(&json_content)?;
        Ok(record)
    }
}

impl QuizHistoryRecord {
    pub fn add_quiz_title(&mut self, quiz_title: String) {
        self.quiz_titles.push(quiz_title);
    }
    pub fn add_quiz_timestamp(&mut self) {
        let quiz_timestamp = Local::now();
        self.quiz_timestamps.push(quiz_timestamp);
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    pub fn show_history(&self) {
        println!("{}", "=== Quiz History ===".bright_yellow().bold());

        if self.quiz_titles.is_empty() || self.quiz_timestamps.is_empty() {
            println!("{}", "No quiz history available.".dimmed());
            return;
        }

        for (i, (title, timestamp)) in self
            .quiz_titles
            .iter()
            .zip(self.quiz_timestamps.iter())
            .enumerate()
        {
            println!(
                "{} {}",
                format!("[{}]", i + 1).bright_blue().bold(),
                format!(
                    "{} at {}",
                    title.green(),
                    timestamp.format("%Y/%m/%d %H:%M:%S").to_string().cyan()
                )
            );
        }

        println!("{}", "====================".bright_yellow().bold());
    }
}

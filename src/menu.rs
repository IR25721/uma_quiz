use crate::{quiz::Quizzes, record::Record};
use color_eyre::Result;
use colored::*;
use proconio::input;
use std::io::{self, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn show_welcome() {
    let msg = "Welcome to Uma_Quiz!";
    for c in msg.chars() {
        print!("{}", c.to_string().green().bold());
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(40));
    }
    println!("\n");
    sleep(Duration::from_millis(300));
}

pub fn show_menu() {
    println!("{}", "╔══════════════════════════╗".yellow());
    println!("{}", "║        Main Menu         ║".yellow().bold());
    println!("{}", "╠══════════════════════════╣".yellow());
    println!("{}", "║ 0: Start Quiz            ║".cyan());
    println!("{}", "║ 1: Show Your Record      ║".cyan());
    println!("{}", "║ 2: Exit                  ║".cyan());
    println!("{}", "╚══════════════════════════╝".yellow());
}

pub fn read_u8(prompt: &str, max: u8) -> u8 {
    loop {
        print!("{}", prompt.bright_blue().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Failed to read input.".red());
            continue;
        }

        match input.trim().parse::<u8>() {
            Ok(n) if n <= max => return n,
            _ => println!("{}", "Invalid input. Please enter a valid number.".red()),
        }
    }
}

pub fn read_usize(prompt: &str) -> usize {
    loop {
        print!("{}", prompt.bright_blue().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Failed to read input.".red());
            continue;
        }

        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            _ => println!("{}", "Invalid input. Please enter a number.".red()),
        }
    }
}

pub fn pause() {
    println!("{}", "\nPress Enter to continue...".dimmed());
    let _ = io::stdin().read_line(&mut String::new());
}
fn is_correct_answers(expected_answers: &[String], user_answers: &[String]) -> bool {
    let mut sorted_expected = expected_answers.to_vec();
    let mut sorted_user = user_answers.to_vec();

    sorted_expected.sort();
    sorted_user.sort();

    sorted_expected == sorted_user
}

pub fn start_quiz(quizzes: Quizzes, mut record: Record, record_path: &Path) -> Result<()> {
    for quiz in quizzes {
        println!("{}", format!("問題: {}", quiz.question).bold());

        println!(
            "{}",
            "答えをカンマ区切りで入力してください:".bright_yellow()
        );
        input! {
            user_input: String
        }
        let user_answers: Vec<String> = user_input
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let is_correct = is_correct_answers(&quiz.answers, &user_answers);
        if is_correct {
            println!("{}", "正解！".green().bold());
            record.add_corrected(user_answers.len());
        } else {
            println!("{}", "不正解！".red().bold());
            println!("{} {:?}", "正しい答え:".bright_green(), quiz.answers);
        }
        record.add_answered(user_answers.len());
        record.save(record_path)?;
        record.print_stats();
        println!("{}", "------------------------------".dimmed());
    }
    Ok(())
}

pub fn show_record(record: Record) {
    println!("{}", "=== Your Quiz Record ===".yellow().bold());
    record.print_stats();
    println!("{}", "=========================".yellow().bold());
}


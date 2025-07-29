mod correct_rate_record;
mod menu;
mod quiz;
mod quiz_history_record;
mod read_file;
use color_eyre::Result;
use colored::*;
use correct_rate_record::CorrectRateRecord;
use menu::{
    choice_quizzes, clear_screen, pause, read_u8, read_usize, show_menu, show_record, show_welcome,
    start_quiz,
};
use quiz::Quizzes;
use quiz_history_record::QuizHistoryRecord;
use read_file::ReadFile;
use std::path::Path;

fn main() -> Result<()> {
    clear_screen();
    show_welcome();

    loop {
        clear_screen();

        let record_path = Path::new("quiz_data/correct_rate_record.json");
        let quiz_history_record_path = Path::new("quiz_data/quiz_history_record.json");
        let record = CorrectRateRecord::new(record_path)?;
        let quiz_history_record = QuizHistoryRecord::new(quiz_history_record_path)?;
        show_menu();

        let choice = read_u8("Your choice > ", 2);

        match choice {
            0 => {
                let path_buf = choice_quizzes()?;
                let data_path = path_buf.as_path();
                let mut quizzes = Quizzes::new(data_path)?;

                quizzes.shuffle();
                let max_quizzes = quizzes.get_quizzes_len();
                let quiz_len = loop {
                    println!(
                        "Enter number of quizzes to attempt (Max: {})",
                        max_quizzes.to_string().yellow()
                    );
                    let input = read_usize("Number > ");
                    if input == 0 {
                        println!("{}", "Number must be at least 1.".red());
                    } else if input > max_quizzes {
                        println!(
                            "{}",
                            "Please enter a number within the maximum limit.".red()
                        );
                    } else {
                        break input;
                    }
                };

                quizzes.get_quizzes(quiz_len);
                start_quiz(
                    quizzes,
                    record,
                    record_path,
                    quiz_history_record,
                    quiz_history_record_path,
                )?;
                pause();
            }
            1 => {
                show_record(record, quiz_history_record);
                pause();
            }
            2 => {
                println!("\n{}", "GoodBye!".green().bold());
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

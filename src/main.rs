mod menu;
mod quiz;
mod read_file;
mod record;

use color_eyre::Result;
use colored::*;
use menu::{
    clear_screen, pause, read_u8, read_usize, show_menu, show_record, show_welcome, start_quiz,
};
use quiz::Quizzes;
use read_file::ReadFile;
use record::Record;
use std::path::Path;

fn main() -> Result<()> {
    clear_screen();
    show_welcome();

    loop {
        clear_screen();

        let data_path = Path::new("quiz_data/keiba_base.json");
        let record_path = Path::new("quiz_data/record.json");
        let mut quizzes = Quizzes::new(data_path)?;
        let record = Record::new(record_path)?;
        quizzes.shuffle();

        show_menu();

        let choice = read_u8("Your choice > ", 2);

        match choice {
            0 => {
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
                start_quiz(quizzes, record, record_path)?;
                pause();
            }
            1 => {
                show_record(record);
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

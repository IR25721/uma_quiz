mod quiz;

use color_eyre::Result;
use proconio::input;
use quiz::Quizzes;
use std::{fmt::Display, path::Path};

struct AnswerResult {
    expected_answer: String,
    user_answer: String,
}

impl AnswerResult {
    fn new_by_ask(expected_answer: String) -> Self {
        input! {
            user_answer: String
        }

        AnswerResult {
            expected_answer,
            user_answer,
        }
    }

    fn is_collect(&self) -> bool {
        self.expected_answer == self.user_answer
    }
}

impl Display for AnswerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] 答え:{}: あなたの答え: {}",
            if self.is_collect() {
                "正解"
            } else {
                "不正解"
            },
            self.expected_answer,
            self.user_answer
        )
    }
}

fn main() -> Result<()> {
    let data_path = Path::new("quiz_data/keiba_base.json");
    let mut quizzes = Quizzes::new(data_path)?;

    quizzes.shuffle();

    for quiz in quizzes {
        println!("問題: {}", quiz.question);

        let answer_results = quiz
            .answers
            .into_iter()
            .map(AnswerResult::new_by_ask)
            .collect::<Vec<AnswerResult>>();

        for result in answer_results {
            println!("{result}");
            println!()
        }
    }

    Ok(())
}

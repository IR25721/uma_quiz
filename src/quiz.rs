use color_eyre::Result;
use color_eyre::eyre::{ContextCompat, bail};
use proconio::input;
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Quiz {
    id: i32,
    title: String,
    question: String,
    answer: Vec<String>,
}

impl Quiz {
    fn get_quizzes() -> Result<Vec<Self>> {
        let path = Path::new("quiz_data/keiba_base.json");
        let json_content = fs::read_to_string(path)?;
        let quizzes: Vec<Self> = serde_json::from_str(&json_content)?;
        Ok(quizzes)
    }

    fn get_quiz(id: i32) -> Result<Self> {
        let quizzes = Quiz::get_quizzes()?;
        quizzes
            .into_iter()
            .find(|quiz| quiz.id == id)
            .context(format!("Quiz with id {} not found", id))
    }

    pub fn get_quiz_random() -> Result<Self> {
        let quizzes = Quiz::get_quizzes().unwrap();
        if let Some(max_id) = quizzes.iter().map(|quiz| quiz.id).max() {
            let mut rng = rand::rng();
            let random_id = rng.random_range(0..=max_id);
            Quiz::get_quiz(random_id)
        } else {
            bail!("クイズデータが空です")
        }
    }

    pub fn get_quiz_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_quiz_question(&self) -> String {
        self.question.clone()
    }

    pub fn get_quiz_answer(&self) -> Vec<String> {
        self.answer.clone()
    }

    fn input_answer(&self) -> Vec<String> {
        let answer_len = self.get_quiz_answer().len();
        let mut input_answer = vec![];
        for i in 0..answer_len {
            println!("あと答えを答えを{}回入力してください．", answer_len - i);
            input! {
                answer:String
            }
            input_answer.push(answer);
        }
        input_answer
    }

    pub fn play_quiz(&self) -> String {
        println!("{}", self.question);
        let input_answer = self.input_answer();
        let correct_set: HashSet<_> = self.answer.iter().map(|s| s.trim()).collect();
        let input_set: HashSet<_> = input_answer.iter().map(|s| s.trim()).collect();
        if correct_set == input_set {
            "正解です!".to_owned()
        } else {
            "不正解です...".to_owned()
        }
    }
}

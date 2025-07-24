use color_eyre::Result;
use rand::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Quiz {
    pub title: String,
    pub question: String,
    pub answers: Vec<String>,
}

#[derive(Debug)]
pub struct Quizzes {
    inner: Vec<Quiz>,
    rng: ThreadRng,
}

impl IntoIterator for Quizzes {
    type Item = Quiz;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Quizzes {
    pub fn new<T>(data_path: T) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        let json_content = fs::read_to_string(data_path)?;
        let quizzes: Vec<Quiz> = serde_json::from_str(&json_content)?;

        let rng = rand::rng();

        Ok(Quizzes {
            inner: quizzes,
            rng,
        })
    }

    pub fn shuffle(&mut self) {
        self.inner.shuffle(&mut self.rng)
    }
}

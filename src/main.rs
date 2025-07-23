use quiz::Quiz;

mod quiz;

fn main() {
    let quiz = Quiz::get_quiz_random().unwrap();
    println!("{}", quiz.play_quiz())
}

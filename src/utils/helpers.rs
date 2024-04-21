use crate::services::feedback_service::{Feedback, FeedbackType};
use std::io::{self, Write};

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_feedback() -> Feedback {
    let feedback_type_str =
        get_user_input("Enter feedback type (p: Positive, n: negative, u: Unspecified): ");
    let feedback_type = match feedback_type_str.as_str() {
        "p" => FeedbackType::Positive,
        "n" => FeedbackType::Negative,
        "u" => FeedbackType::Unspecified,
        _ => {
            println!("Invalid feedback type");
            return get_feedback();
        }
    };
    let feedback_message = get_user_input("Enter your feedback message: ");
    Feedback::new(feedback_type, feedback_message)
}
pub fn get_decision(prompt: &str) -> bool {
    let decision = get_user_input(prompt);
    decision.to_lowercase() != "n"
}

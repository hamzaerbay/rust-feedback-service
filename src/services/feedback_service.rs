pub enum FeedbackType {
    Positive,
    Negative,
    Unspecified,
}

pub struct Feedback {
    feedback_type: FeedbackType,
    message: String,
}

impl Feedback {
    pub fn new(feedback_type: FeedbackType, message: String) -> Self {
        Feedback {
            feedback_type,
            message,
        }
    }
}

pub struct FeedbackService {
    feedbacks: Vec<Feedback>,
}

impl FeedbackService {
    pub fn new() -> Self {
        FeedbackService {
            feedbacks: Vec::new(),
        }
    }
    pub fn add_feedback(&mut self, feedback: Feedback) {
        self.feedbacks.push(feedback)
    }
    pub fn list_all_feedbacks(&self) {
        for feedback in &self.feedbacks {
            let feedback_type = match feedback.feedback_type {
                FeedbackType::Positive => "Positive",
                FeedbackType::Negative => "Negative",
                FeedbackType::Unspecified => "Unspecified",
            };
            println!(
                "- Feedback type: {}, Message {}",
                feedback_type, feedback.message
            )
        }
    }
}

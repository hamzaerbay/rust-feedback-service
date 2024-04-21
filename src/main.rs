mod services;
mod utils;
use services::feedback_service::FeedbackService;
use utils::helpers::{get_decision, get_feedback};

fn main() {
    let mut service = FeedbackService::new();

    loop {
        let feedback_message = get_feedback();
        service.add_feedback(feedback_message);


        let decision = get_decision("Enter Y to add more feedback or N to stop: ");
        if !decision {
            break;
        }
        // TODO: provide proper cli navigation
        let should_list_all = get_decision("Enter Y to list all feedbacks or N to stop: ");
        if should_list_all {
            service.list_all_feedbacks();
        }
        
    }
}

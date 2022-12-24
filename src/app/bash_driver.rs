use super::{db::ToDo, ActionResponse, ActionResponseType};

impl ToDo {
    pub fn show_as_check(&self) -> String {
        match &self.done {
            true => "☑".to_string(),
            false => "☐".to_string(),
        }
    }
}

pub fn display_action_response(res: &ActionResponse) {
    if res._type == ActionResponseType::Error {
        print!("❌ ");
    }
    if res._type == ActionResponseType::Success {
        print!("👍 ");
    }
    if res.message.len() > 0 {
        println!("{}", res.message);
    }
    if res._type == ActionResponseType::Content {
        match &res.todo {
            Some(t) => println!("{} {}", t.show_as_check(), t.description),
            None => {}
        }
    }
}

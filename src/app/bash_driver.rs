use super::{db::Dro, ActionResponse, ActionResponseType};

impl Dro {
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
        println!(" {}", res.message);
    }
    if res._type == ActionResponseType::Content {

        if res.formatting.is_some() && res.formatting.unwrap().opts.into_iter().

        match &res.dros {
            Some(t) => println!("{} {}", t.show_as_check(), t.description),
            None => {}
        }
    }
}

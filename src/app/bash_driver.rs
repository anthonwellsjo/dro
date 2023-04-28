use super::{db::Dro, ActionResponse, ActionResponseType, Opt};

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
        if res.formatting.is_some() {
            match res.formatting.as_ref().unwrap().opts.first().unwrap() {
                Opt::Day => {
                    for dro in res.dros.as_ref().unwrap().into_iter() {
                        println!(
                            "{} {} {}",
                            dro.show_as_check(),
                            dro.created.as_ref().unwrap(),
                            dro.description
                        )
                    }
                }
            }
        } else {
            for dro in res.dros.as_ref().unwrap().into_iter() {
                println!("{} {}", dro.show_as_check(), dro.description,)
            }
        }
    }
}

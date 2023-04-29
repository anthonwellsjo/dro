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
        for (index, dro) in res.dros.as_ref().unwrap().into_iter().enumerate() {
            if res.formatting.is_some() {
                let opts = &res.formatting.as_ref().unwrap().opts;
                for opt in opts.into_iter() {
                    match opt {
                    Opt::Index => {
                            print!("{} ", index)
                        }
                    Opt::Day => {
                        print!("{} ", dro.created.as_ref().unwrap(),)
                    }
                }
                }
            }
            println!("{} {}", dro.show_as_check(), dro.description,)
        }
    }
}

use crate::app::utils::get_action;

use self::utils::get_argument;

mod db;
pub mod utils;

#[derive(Debug)]
pub enum Action {
    v,
    a,
}

#[derive(Debug)]
struct InputError;

impl Action {
    fn from_string(s: &str) -> Result<Action, InputError> {
        match s {
            "v" => Ok(Action::v),
            "a" => Ok(Action::a),
            _ => Err(InputError),
        }
    }
}

pub struct Session {
    args: Vec<String>,
}

impl Session {
    pub fn new(args: Vec<String>) -> Self {
        Session { args }
    }
    fn add_todo(&mut self) {
        let description = get_argument(&mut self.args)
            .expect("Error while getting the description for the todo.");
        let to_do = db::ToDo {
            description: description.to_string(),
            done: false,
        };
        db::save_todo_to_db(to_do).expect("A problem occured while saving to todo");
    }
    pub fn run(&mut self) {
        let action = get_action(&self.args);
        match action {
            Action::v => {}
            Action::a => {
                self.add_todo();
            }
        }
        println!("{:?}", &self.action);
    }
}

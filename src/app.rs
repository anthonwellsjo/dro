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
    fn get_action(&mut self) -> Action {
        Action::from_string(self.args.first().unwrap()).expect("Argument error!")
    }
    pub fn run(&mut self) {
        self.get_action();
        match self.action {
            Action::v => {}
            Action::a => {}
        }
        println!("{:?}", &self.action);
    }
}

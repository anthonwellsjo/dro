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
    action: Action,
}

impl Session {
    pub fn new(self, args: Vec<String>) -> Self {
        Session {
            args,
            action: self.get_action(),
        }
    }
    fn add_todo(){
        todo!()
    }
    fn get_action(&mut self) -> Action {
        Action::from_string(self.args.first().unwrap()).expect("Argument error!")
    }
    pub fn run(&mut self) {
        self.get_action();
        println!("{:?}", &self.actions);
    }
}

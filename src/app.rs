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
    fn get_action(&mut self){
        for x in self.args.iter() {
            let action = Action::from_string(x).expect("Argument error!");
            self.actions.push(action);
        }

    }
    pub fn run(&mut self) {
        self.get_action();
        println!("{:?}", &self.actions);
    }
}

use crate::app::utils::get_action;

use self::utils::get_argument;

mod db;
pub mod utils;

#[derive(Debug)]
pub enum Action {
    View,
    Add,
    Markdone
}

#[derive(Debug)]
struct InputError;

impl Action {
    fn from_string(s: &str) -> Result<Action, InputError> {
        match s {
            "v" | "view" => Ok(Action::View),
            "a" | "add" => Ok(Action::Add),
            "md" | "markdone" => Ok(Action::Markdone),
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
            Action::View => {
                self.show_todos();
            }
            Action::Add => {
                self.add_todo();
            }
            Action::Markdone => {
                self.markdone();
            }
        }
    }

    fn show_todos(&self) {
        let todos = db::get_todos().expect("Error while getting todos.");

        for (index, todo) in todos.iter().enumerate() {
            println!("{} {} {}", index, todo.show_as_check(), todo.description)
        }
    }

    fn markdone(&mut self) {
        let arg = get_argument(&mut self.args).expect("You forgot to add which dro to mark as done."); 
        println!("You are deleting dro no {}", arg);
    }
}

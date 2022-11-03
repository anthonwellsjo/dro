use crate::app::utils::{get_action, get_md_or_mu_index_argument};

use self::utils::get_argument;

mod db;
pub mod utils;

#[derive(Debug)]
pub enum Action {
    View,
    Add,
    MarkAsDone,
    MarkAsUndone,
}

#[derive(Debug)]
struct InputError;

impl Action {
    fn from_string(s: &str) -> Result<Action, InputError> {
        match s {
            "v" | "view" => Ok(Action::View),
            "a" | "add" => Ok(Action::Add),
            "md" | "markdone" => Ok(Action::MarkAsDone),
            "mu" | "markundone" => Ok(Action::MarkAsUndone),
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
            Action::MarkAsDone => {
                self.mark_as_done();
            }
            Action::MarkAsUndone => {
                self.mark_as_undone();
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
        let arg = get_md_or_mu_index_argument(&mut self.args).unwrap();
        let todos = db::get_todos().expect("Error while getting todos.");
        let description: &str;
        match &todos.get(arg) {
            Some(todo) => description = &todo.description,
            None => {
                println!("Could not find any dro on index {}", arg);
                return;
            }
        }

        match db::mark_todo_as_done(description) {
            Ok(()) => println!("Updated dro on index {} successfully.", arg),
            Err(error) => println!("Could not update dro at porsition {}: {}", arg, error),
        }
    }
}

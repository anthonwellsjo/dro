use crate::app::bash_driver::{get_action, get_md_or_mu_index_argument};

use self::bash_driver::get_argument;

mod db;
pub mod bash_driver;

#[derive(Debug, PartialEq)]
pub enum Action {
    View,
    Add,
    MarkAsDone,
    MarkAsUndone,
    Purge,
    Help,
}

#[derive(Debug)]
struct InputError;


pub struct Session {
    args: Vec<String>,
}

impl Session {

    pub fn run(action: Action, arugment: &str) {
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
            Action::Purge => {
                self.purge_todos();
            }
            Action::Help => {
                self.show_help();
            }
        }
    }

    fn add_todo(&mut self) {
        let description = get_argument(&mut self.args)
            .expect("Error while getting the description for the todo.");
        let to_do = db::ToDo::new(description);
        db::save_todo_to_db(to_do).expect("A problem occured while saving to todo");
        println!("dro added.");
    }

    fn show_todos(&self) {
        let todos = db::get_todos().expect("Error while getting todos.");

        for (index, todo) in todos.iter().enumerate() {
            println!("{} {} {}", index, todo.show_as_check(), todo.description)
        }
    }

    fn mark_as_done(&mut self) {
        let arg = get_md_or_mu_index_argument(&mut self.args).unwrap();
        let todos = db::get_todos().expect("Error while getting todos.");
        let description: &str;
        match &todos.get(arg) {
            Some(todo) => description = &todo.description,
            None => {
                println!("there is no dro on index {}.", arg);
                return;
            }
        }

        match db::mark_todo_as_done(description) {
            Ok(()) => println!("dro on index {} updated.", arg),
            Err(error) => println!("could not update dro at porsition {}: {}", arg, error),
        }
    }

    fn mark_as_undone(&mut self) {
        let arg = get_md_or_mu_index_argument(&mut self.args).unwrap();
        let todos = db::get_todos().expect("Error while getting todos.");
        let description: &str;
        match &todos.get(arg) {
            Some(todo) => description = &todo.description,
            None => {
                println!("there is no dro on index {}.", arg);
                return;
            }
        }

        match db::mark_todo_as_undone(description) {
            Ok(()) => println!("dro on index {} updated.", arg),
            Err(error) => println!("could not update dro at porsition {}: {}", arg, error),
        }
    }

    fn purge_todos(&self) {
        db::purge_todos().expect("A problem occured while purging.");
        println!("dros have been purged.");
    }

    fn show_help(&self) {
        println!(
            "
            Command:        Argument:

            v, view         -                   View all todos
            a, add          description         Add new dro with <description>
            md, markdone    index               Mark dro at position <index> as done
            mu, markundone  index               Mark dro at position <index> as undone
            pu, purge       -                   Deletes all dros that are marked as done
            h, help         -                   See documentation
        "
        );
    }
}

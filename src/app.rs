use crate::app::bash_driver:: get_md_or_mu_index_argument;
use self::db::ToDo;


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

enum ActionResponseType {
    Error,
    Success,
}


struct ActionResponse<'a> {
    message: &'a str,
    res_type: ActionResponseType,
    todo: Option<&'a ToDo>
}

#[derive(Debug)]
struct InputError;


pub struct Session<'a> {
    action_responses: Vec<ActionResponse<'a>>
}

impl Session<'_> {

    pub fn new(self) -> Self {Session{action_responses: vec!{}}}

    pub fn run(self, action: Action, argument: &str) -> Vec<ActionResponse> {
        match action {
            Action::View => {
                self.show_todos();
            }
            Action::Add => {
                self.add_todo(action, argument);
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

    self.action_responses
    }

    fn add_todo(&self, action: Action, argument: &str) {
        let todo = db::ToDo::new(argument);
        match db::save_todo_to_db(todo) {
            Ok(_) => (),
            Err(_) => {
            self.action_responses.push(ActionResponse { message:"database didn't want to save this todo" , res_type: ActionResponseType::Error, todo: None })
        },
        };
        
        self.action_responses.push(ActionResponse { message:"dro added" , res_type: ActionResponseType::Success, todo: Some(&todo) });
    }

    fn show_todos(&self) {
        let todos = db::get_todos();

        match todos {
            Ok(todos) => {
            for (index, todo) in todos.iter().enumerate(){
                self.action_responses.push(ActionResponse { message: "" , res_type: ActionResponseType::Success, todo: Some(todo) })
            }
        }
            Err(_) => todo!(),
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

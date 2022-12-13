use crate::app::bash_driver:: get_md_or_mu_index_argument;


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
    Success
}


struct ActionResponse<'a> {
    text: &'a str,
    res_type: ActionResponseType
}

#[derive(Debug)]
struct InputError;


pub struct Session<'a> {
    action_responses: Vec<ActionResponse<'a>>
}

impl Session<'_> {

    pub fn new(self) -> Self {Session{action_responses: vec!{}}}

    pub fn run(self, action: Action, argument: &str) -> ActionResponse {
        match action {
            Action::View => {
                self.show_todos()
            }
            Action::Add => {
                self.add_todo(action, argument)
            }
            Action::MarkAsDone => {
                self.mark_as_done()
            }
            Action::MarkAsUndone => {
                self.mark_as_undone()
            }
            Action::Purge => {
                self.purge_todos()
            }
            Action::Help => {
                self.show_help()
            }
        }
    }

    fn add_todo(&self, action: Action, argument: &str) -> ActionResponse {
        let to_do = db::ToDo::new(argument);
        db::save_todo_to_db(to_do).unwrap_or_else(|_|{
            self.action_responses.push(ActionResponse { text:"a problem occured while saving to todo." , res_type: ActionResponseType::Error });
            self.action_responses
        }
        )
        self.action_responses.push(ActionResponse { text:"dro added" , res_type: ActionResponseType::Success })

        println!("dro added.");
    }

    fn show_todos(&self) -> ActionResponse {
        let todos = db::get_todos().expect("Error while getting todos.");

        for (index, todo) in todos.iter().enumerate() {
            println!("{} {} {}", index, todo.show_as_check(), todo.description)
        }
    }

    fn mark_as_done(&mut self) -> ActionResponse {
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

        match db::mark_todo_as_done(description) -> ActionResponse {
            Ok(()) => println!("dro on index {} updated.", arg),
            Err(error) => println!("could not update dro at porsition {}: {}", arg, error),
        }
    }

    fn mark_as_undone(&mut self) -> ActionResponse {
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

        match db::mark_todo_as_undone(description) -> ActionResponse {
            Ok(()) => println!("dro on index {} updated.", arg),
            Err(error) => println!("could not update dro at porsition {}: {}", arg, error),
        }
    }

    fn purge_todos(&self) -> ActionResponse {
        db::purge_todos().expect("A problem occured while purging.");
        println!("dros have been purged.");
    }

    fn show_help(&self) -> ActionResponse {
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

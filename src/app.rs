use self::db::ToDo;

pub mod bash_driver;
mod db;

#[derive(Debug, PartialEq)]
pub enum Action {
    View,
    Add,
    MarkAsDone,
    MarkAsUndone,
    Purge,
    Help,
    Version,
}
impl Action {
    pub fn from_string(s: &str) -> Option<Action> {
        match s {
            "s" | "see" => Some(Action::View),
            "a" | "add" => Some(Action::Add),
            "md" | "markdone" => Some(Action::MarkAsDone),
            "mu" | "markundone" => Some(Action::MarkAsUndone),
            "pu" | "purge" => Some(Action::Purge),
            "h" | "help" => Some(Action::Help),
            "v" | "version" => Some(Action::Version),
            _ => None,
        }
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ActionResponseType {
    Error,
    Success,
    Content
}

#[derive(Debug)]
pub struct ActionResponse<'a> {
    pub message: &'a str,
    pub _type: ActionResponseType,
    pub todo: Option<ToDo>,
}

pub struct Session<'a> {
    pub action_responses: Vec<ActionResponse<'a>>,
}

impl Session<'_> {
    pub fn new() -> Self {
        Session {
            action_responses: vec![],
        }
    }

    pub fn run(&mut self, action: Option<Action>, argument: Option<String>) {
        match action {
            Some(Action::View) => {
                self.show_todos();
            }
            Some(Action::Purge) => {
                self.purge_todos();
            }
            Some(Action::Help) => {
                self.show_help();
            }
            Some(Action::Version) => {
                self.show_version();
            }

            // todo: make wrapper for all actions that need arg
            Some(Action::Add) => {
                match argument {
                    Some(arg) => {
                        self.add_todo(&arg);
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.",
                            _type: ActionResponseType::Error,
                            todo: None,
                        });
                    }
                };
            }
            Some(Action::MarkAsDone) => {
                match argument {
                    Some(arg) => {
                        self.mark_as_done(&arg);
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.",
                            _type: ActionResponseType::Error,
                            todo: None,
                        });
                    }
                };
            }
            Some(Action::MarkAsUndone) => {
                match argument {
                    Some(arg) => {
                        self.mark_as_undone(&arg);
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.",
                            _type: ActionResponseType::Error,
                            todo: None,
                        });
                    }
                };
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?",
                    _type: ActionResponseType::Success,
                    todo: None,
                });
            }
        }
    }

    fn add_todo(&mut self, argument: &str) {
        let todo = db::ToDo::new(argument);
        match db::save_todo_to_db(&todo) {
            Ok(_) => &(),
            Err(_) => &self.action_responses.push(ActionResponse {
                message: "database didn't want to save this todo",
                _type: ActionResponseType::Error,
                todo: None,
            }),
        };

        self.action_responses.push(ActionResponse {
            message: "dro added",
            _type: ActionResponseType::Success,
            todo: Some(todo),
        });
    }

    fn show_todos(&mut self) {
        let todos = db::get_todos();

        match todos {
            Ok(todos) => {
                for (_, todo) in todos.into_iter().enumerate() {
                    self.action_responses.push(ActionResponse {
                        message: "",
                        _type: ActionResponseType::Content,
                        todo: Some(todo),
                    });
                }
            }
            Err(_) => todo!(),
        }
    }

    fn get_index_from_arg(&mut self, arg: &str) -> Option<usize> {
        match arg.trim().parse::<usize>() {
            Ok(i) => Some(i),
            Err(_) => {
                self.action_responses.push(ActionResponse {
                    message: "couldn't parse argument to index number",
                    _type: ActionResponseType::Error,
                    todo: None,
                });
                None
            }
        }
    }

    fn get_todo_from_index(&mut self, index: &usize, todos: Vec<ToDo>) -> Option<ToDo> {
        match todos.into_iter().nth(*index) {
            Some(todo) => Some(todo),
            None => {
                self.action_responses.push(ActionResponse {
                    message: "there is no dro on that index",
                    _type: ActionResponseType::Error,
                    todo: None,
                });
                None
            }
        }
    }

    fn mark_as_done(&mut self, arg: &str) -> Option<()> {
        let index: &usize = &self.get_index_from_arg(arg)?;
        let todos: Vec<ToDo> = db::get_todos().expect("fatal error while getting todos.");
        let todo: ToDo = self.get_todo_from_index(index, todos)?;

        db::mark_todo_as_done(&todo.description)
            .expect(&("could not update dro at position ".to_owned() + &arg));
        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            todo: Some(todo),
        });
        Some(())
    }

    fn mark_as_undone(&mut self, arg: &str) -> Option<()> {
        let index: usize = self.get_index_from_arg(arg)?;
        let todos: Vec<ToDo> = db::get_todos().expect("fatal error while getting todos.");
        let todo = self.get_todo_from_index(&index, todos)?;

        db::mark_todo_as_undone(&todo.description)
            .expect(&("could not update dro at position ".to_owned() + &arg));
        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            todo: Some(todo),
        });
        Some(())
    }

    fn purge_todos(&mut self) {
        db::purge_todos().expect("A problem occured while purging.");
        self.action_responses.push(ActionResponse {
            message: "dros have been purged.",
            _type: ActionResponseType::Success,
            todo: None,
        });
    }

    fn show_help(&mut self) {
        self.action_responses.push(ActionResponse {
            message: "
            Command:        Argument:

            s, see          -                   View all todos
            a, add          description         Add new dro with <description>
            md, markdone    index               Mark dro at position <index> as done
            mu, markundone  index               Mark dro at position <index> as undone
            pu, purge       -                   Deletes all dros that are marked as done
            h, help         -                   See documentation
            v, version      -                   See current version
            ",
            _type: ActionResponseType::Content,
            todo: None,
        });
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message:  env!("CARGO_PKG_VERSION"),
            _type: ActionResponseType::Content,
            todo: None,
        });
    }
}

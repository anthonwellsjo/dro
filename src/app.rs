use self::db::Dro;

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
    pub dro: Option<Dro>,
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
                self.show_dros();
            }
            Some(Action::Purge) => {
                self.purge_dros();
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
                        self.add_dro(&arg);
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.",
                            _type: ActionResponseType::Error,
                            dro: None,
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
                            dro: None,
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
                            dro: None,
                        });
                    }
                };
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?",
                    _type: ActionResponseType::Success,
                    dro: None,
                });
            }
        }
    }

    fn add_dro(&mut self, argument: &str) {
        let dro = db::Dro::new(argument);
        match db::save_dro_to_db(&dro) {
            Ok(_) => &(),
            Err(_) => &self.action_responses.push(ActionResponse {
                message: "database didn't want to save this dro",
                _type: ActionResponseType::Error,
                dro: None,
            }),
        };

        self.action_responses.push(ActionResponse {
            message: "dro added",
            _type: ActionResponseType::Success,
            dro: Some(dro),
        });
    }

    fn show_dros(&mut self) {
        let dros = db::get_dros();

        match dros {
            Ok(dros) => {
                for (_, dro) in dros.into_iter().enumerate() {
                    self.action_responses.push(ActionResponse {
                        message: "",
                        _type: ActionResponseType::Content,
                        dro: Some(dro),
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
                    dro: None,
                });
                None
            }
        }
    }

    fn get_dro_from_index(&mut self, index: &usize, dros: Vec<Dro>) -> Option<Dro> {
        match dros.into_iter().nth(*index) {
            Some(dro) => Some(dro),
            None => {
                self.action_responses.push(ActionResponse {
                    message: "there is no dro on that index",
                    _type: ActionResponseType::Error,
                    dro: None,
                });
                None
            }
        }
    }

    fn mark_as_done(&mut self, arg: &str) -> Option<()> {
        let index: &usize = &self.get_index_from_arg(arg)?;
        let dros: Vec<Dro> = db::get_dros().expect("fatal error while getting dros.");
        let dro: Dro = self.get_dro_from_index(index, dros)?;

        db::mark_dro_as_done(&dro.description)
            .expect(&("could not update dro at position ".to_owned() + &arg));
        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            dro: Some(dro),
        });
        Some(())
    }

    fn mark_as_undone(&mut self, arg: &str) -> Option<()> {
        let index: usize = self.get_index_from_arg(arg)?;
        let dros: Vec<Dro> = db::get_dros().expect("fatal error while getting dros.");
        let dro = self.get_dro_from_index(&index, dros)?;

        db::mark_dro_as_undone(&dro.description)
            .expect(&("could not update dro at position ".to_owned() + &arg));
        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            dro: Some(dro),
        });
        Some(())
    }

    fn purge_dros(&mut self) {
        db::purge_dros().expect("A problem occured while purging.");
        self.action_responses.push(ActionResponse {
            message: "dros have been purged.",
            _type: ActionResponseType::Success,
            dro: None,
        });
    }

    fn show_help(&mut self) {
        self.action_responses.push(ActionResponse {
            message: "
            Command:        Argument:

            s, see          -                   View all dros
            a, add          description         Add new dro with <description>
            md, markdone    index               Mark dro at position <index> as done
            mu, markundone  index               Mark dro at position <index> as undone
            pu, purge       -                   Deletes all dros that are marked as done
            h, help         -                   See documentation
            v, version      -                   See current version
            ",
            _type: ActionResponseType::Content,
            dro: None,
        });
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message:  env!("CARGO_PKG_VERSION"),
            _type: ActionResponseType::Content,
            dro: None,
        });
    }
}

use self::db::Dro;

pub mod bash_driver;
mod db;

#[derive(Debug, PartialEq)]
pub enum Action {
    See,
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
            "s" | "see" => Some(Action::See),
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

#[derive(Debug, PartialEq)]
pub enum Opt {
    Day,
}
impl Opt {
    pub fn from_string(s: &str) -> Option<Opt> {
        match s {
            "day" => Some(Opt::Day),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Formatting,
}
impl Flag {
    pub fn from_string(s: &str) -> Option<Flag> {
        match s {
            "-f" => Some(Flag::Formatting),
            _ => None,
        }
    }
    pub fn options(self: Self) -> Vec<Opt> {
        match self {
            Flag::Formatting => vec![Opt::Day],
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ActionResponseType {
    Error,
    Success,
    Content,
}
#[derive(Debug, PartialEq)]
pub enum Formatting {
    Day,
}

#[derive(Debug)]
pub struct ActionResponse<'a> {
    pub formatting: Option<FlagWithOpts>,
    pub message: &'a str,
    pub _type: ActionResponseType,
    pub dros: Option<Vec<Dro>>,
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

    pub fn run(
        &mut self,
        action: Option<Action>,
        argument: Option<String>,
        flags: Vec<Vec<String>>,
    ) {
        let flags = self.parse_flags(&flags);

        match action {
            Some(Action::See) => {
                self.show_dros(flags);
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
                            dros: None,
                            formatting: None,
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
                            dros: None,
                            formatting: None,
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
                            dros: None,
                            formatting: None,
                        });
                    }
                };
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?",
                    _type: ActionResponseType::Success,
                    dros: None,
                    formatting: None,
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
                dros: None,
                formatting: None,
            }),
        };

        self.action_responses.push(ActionResponse {
            message: "dro added",
            _type: ActionResponseType::Success,
            dros: Some(vec!(dro)),
            formatting: None,
        });
    }

    fn show_dros(&mut self, flags: Option<Vec<FlagWithOpts>>) {
        let dros = db::get_dros();


        let formatting = get_formatting(flags);
        match dros {
            Ok(dros) => {
                self.action_responses.push(ActionResponse {
                    message: "",
                    _type: ActionResponseType::Content,
                    dros: Some(dros),
                    formatting
                });
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
                    dros: None,
                    formatting: None,
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
                    dros: None,
                    formatting: None,
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
            dros: Some(vec!(dro)),
            formatting: None,
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
            dros: Some(vec!(dro)),
            formatting: None,
        });
        Some(())
    }

    fn purge_dros(&mut self) {
        db::purge_dros().expect("A problem occured while purging.");
        self.action_responses.push(ActionResponse {
            message: "dros have been purged.",
            _type: ActionResponseType::Success,
            dros: None,
            formatting: None,
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
            dros: None,
            formatting: None,
        });
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message: env!("CARGO_PKG_VERSION"),
            _type: ActionResponseType::Content,
            dros: None,
            formatting: None,
        });
    }

    fn parse_flags(&mut self, flags_with_opts: &Vec<Vec<String>>) -> Option<Vec<FlagWithOpts>> {
        let mut res: Vec<FlagWithOpts> = vec![];
        for flag_with_opts in flags_with_opts.iter() {
            let mut iter = flag_with_opts.into_iter();
            let mut fwo;

            fwo = FlagWithOpts {
                flag: Flag::from_string(iter.next().unwrap()).unwrap(),
                opts: vec![],
            };

            for item in iter {
                let opt = Opt::from_string(&item);
                if opt.is_none() {
                    self.action_responses.push(ActionResponse {
                        formatting: None,
                        message: "Unknown option ",
                        _type: ActionResponseType::Error,
                        dros: None,
                    });
                    continue;
                };

                fwo.opts.push(opt.unwrap());
            }
            res.push(fwo);
        }
        Some(res)
    }
}

fn get_formatting(flags: Option<Vec<FlagWithOpts>>) -> Option<FlagWithOpts> {
    if flags.is_none() {
        return None;
    }
    flags.unwrap().into_iter().find(|f| f.flag == Flag::Formatting)
}

#[derive(Debug, PartialEq)]
pub struct FlagWithOpts {
    pub flag: Flag,
    pub opts: Vec<Opt>,
}

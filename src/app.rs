use std::fs;

use self::db::Dro;

pub mod bash_driver;
mod db;

#[derive(Debug, PartialEq)]
pub enum Action {
    List,
    Add,
    MarkAsDone,
    MarkAsUndone,
    Purge,
    Help,
    Version,
    Init,
}
impl Action {
    pub fn from_string(s: &str) -> Option<Action> {
        match s {
            "init" => Some(Action::Init),
            "ls" | "list" | "see" => Some(Action::List),
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
    Date,
    Index,
    Default,
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Formatting,
    Index,
}
impl Flag {
    pub fn from_string(s: &str) -> Option<Flag> {
        match s {
            "-f" => Some(Flag::Formatting),
            "-i" => Some(Flag::Index),
            _ => None,
        }
    }
    pub fn allowed_options(self: Self) -> Vec<Opt> {
        match self {
            Flag::Formatting => vec![Opt::Date, Opt::Index],
            Flag::Index => vec![],
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
pub enum Formatting {}
impl Formatting {
    pub fn from_string(s: &str) -> Option<Opt> {
        match s {
            "date" | "d" => Some(Opt::Date),
            "index" | "i" => Some(Opt::Index),
            _ => None,
        }
    }
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
        arguments: Option<Vec<String>>,
        flags: Vec<Vec<String>>,
    ) {
        let flags = self.parse_flags(&flags);

        match action {
            Some(Action::Init) => {
                self.init(flags);
            }
            Some(Action::List) => {
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
                match arguments {
                    Some(args) => {
                        self.add_dro(&args);
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
                match arguments {
                    Some(args) => {
                        self.mark_as_done(&args, flags);
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
                match arguments {
                    Some(arg) => {
                        self.mark_as_undone(&arg, flags);
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

    fn init(&mut self, _flags: Option<Vec<FlagWithOpts>>) {
        db::create_local_db().unwrap();
        self.action_responses.push(ActionResponse {
                    message: "created a local db for this folder.",
                    _type: ActionResponseType::Success,
                    dros: None,
                    formatting: None,
                });
    }

    fn add_dro(&mut self, args: &Vec<String>) {
        let mut dros = vec![];
        for argument in args.into_iter() {
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
            dros.push(dro);
        }

        self.action_responses.push(ActionResponse {
            message: "dro added",
            _type: ActionResponseType::Success,
            dros: Some(dros),
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
                    formatting,
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

    fn get_dro_from_query(&self, query: &str, dros: Vec<Dro>) -> Option<Dro> {
        dros.into_iter()
            .find(|dro| dro.description.contains(&query.to_string()))
    }

    fn mark_as_done(&mut self, args: &Vec<String>, flags: Option<Vec<FlagWithOpts>>) -> Option<()> {
        let dros: Vec<Dro> = db::get_dros().expect("fatal error while getting dros.");
        let mut dros_to_update: Vec<Dro> = vec![];
        if flags.is_some() && flags.as_deref().unwrap().len() > 0 {
            println!("flags: {:?}", flags);
            let flag = flags.as_deref().unwrap().first().unwrap();

            match flag.flag {
                Flag::Index => {
                    for arg in args.clone().iter() {
                        let index: &usize = &self.get_index_from_arg(arg)?;
                        dros_to_update.push(self.get_dro_from_index(index, dros.clone())?);
                    }
                }
                _ => {
                    self.action_responses.push(ActionResponse {
                        formatting: None,
                        message: "Flag not valid for this action.",
                        _type: ActionResponseType::Error,
                        dros: None,
                    });
                    return None;
                }
            }
        } else {
            for arg in args.iter() {
                dros_to_update.push(self.get_dro_from_query(arg, dros.clone())?);
            }
        }

        for dro in dros_to_update.clone() {
            db::mark_dro_as_done(&dro.description)
                .expect(&("could not update dro ".to_owned() + &dro.description));
        }

        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            dros: Some(dros_to_update),
            formatting: None,
        });
        Some(())
    }

    fn mark_as_undone(
        &mut self,
        args: &Vec<String>,
        flags: Option<Vec<FlagWithOpts>>,
    ) -> Option<()> {
        let dros: Vec<Dro> = db::get_dros().expect("fatal error while getting dros.");
        let mut dros_to_update: Vec<Dro> = vec![];
        if flags.is_some() && flags.as_deref().unwrap().len() > 0 {
            let flag = flags.as_deref().unwrap().first().unwrap();

            match flag.flag {
                Flag::Index => {
                    for arg in args.iter() {
                        let index: &usize = &self.get_index_from_arg(arg)?;
                        dros_to_update.push(self.get_dro_from_index(index, dros.clone())?);
                    }
                }
                _ => {
                    self.action_responses.push(ActionResponse {
                        formatting: None,
                        message: "Flag not valid for this action.",
                        _type: ActionResponseType::Error,
                        dros: None,
                    });
                    return None;
                }
            }
        } else {
            for arg in args.iter() {
                dros_to_update.push(self.get_dro_from_query(arg, dros.clone())?);
            }
        }

        for dro in dros_to_update.clone() {
            db::mark_dro_as_undone(&dro.description)
                .expect(&("could not update dro ".to_owned() + &dro.description));
        }

        self.action_responses.push(ActionResponse {
            message: "dro updated",
            _type: ActionResponseType::Success,
            dros: Some(dros_to_update),
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
            Command:         Flag:      Argument:
                             
            ls, list         -f <opt>   -             view all dros
            a, add                      description   add new dro with <description>
            md, markdone     -i         query/index   mark dro at position <index> as done
            mu, markundone   -i         query/index   mark dro at position <index> as undone
            pu, purge                   -             deletes all dros that are marked as done
            h, help                     -             see documentation
            v, version                  -             see current version


            Flag options:

            -f/-format: d/date, i/index
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
                let opt;

                match fwo.flag {
                    Flag::Formatting => {
                        opt = Formatting::from_string(&item);
                        if opt.is_none() {
                            self.action_responses.push(ActionResponse {
                                formatting: None,
                                message: "Unknown option ",
                                _type: ActionResponseType::Error,
                                dros: None,
                            });
                            continue;
                        };
                    }
                    Flag::Index => opt = None,
                }

                fwo.opts.push(opt.unwrap_or_else(|| Opt::Default));
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
    flags
        .unwrap()
        .into_iter()
        .find(|f| f.flag == Flag::Formatting)
}

#[derive(Debug, PartialEq)]
pub struct FlagWithOpts {
    pub flag: Flag,
    pub opts: Vec<Opt>,
}

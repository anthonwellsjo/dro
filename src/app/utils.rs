use super::Action;
use std::process;

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}
pub fn get_action(args: &Vec<String>) -> Action {
    Action::from_string(args.first().unwrap_or_else(|| {
        println!("Found no action. Run `help` for documentation.");
        process::exit(1)
    }))
    .expect("Argument error! No action.")
}
pub fn get_argument(args: &mut Vec<String>) -> Option<&str> {
    match args.get(1) {
        Some(x) => Some(x),
        None => {
            println!("Argument missing. Run `help` for documentation.");
            process::exit(1)
        }
    }
}
pub fn get_md_or_mu_index_argument(args: &mut Vec<String>) -> Option<usize> {
    match args.get(1) {
        Some(x) => Some(
            x.trim()
                .parse()
                .expect("Expected a number as second argument."),
        ),
        None => {
            println!("Argument missing. Run `help` for documentation.");
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_action;
    use crate::app::{utils::get_argument, Action};

    #[test]
    fn get_action_from_args() {
        let args: Vec<String> = vec!["md".to_string()];
        let action = get_action(&args);
        println!("{:?}", action);
        assert_eq!(action, Action::MarkAsDone);
    }

    #[test]
    fn get_argument_from_args() {
        let desc = String::from("this is an argument");
        let mut args: Vec<String> = vec!["md".to_string(), desc.clone()];
        let argument = get_argument(&mut args).unwrap();
        println!("{:?}", argument);
        assert_eq!(argument, desc);
    }
}

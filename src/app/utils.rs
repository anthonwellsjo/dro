use super::Action;

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}
pub fn get_action(args: &Vec<String>) -> Action {
    Action::from_string(args.first().unwrap()).expect("Argument error! No action.")
}
pub fn get_argument(args: &mut Vec<String>) -> Option<&str> {
    match args.get_mut(1) {
        Some(x) => Some(x),
        None => {
            panic!("Argument missing.")
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_action_from_args() {
        todo!()
    }
    #[test]
    fn get_argument_from_args() {
        todo!()
    }
}

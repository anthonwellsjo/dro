use app::bash_driver::display_action_response;

mod app;

fn main() {
    let action = brr::get_argument_at(0).unwrap();
    let action = app::Action::from_string(&action);
    let argument = brr::get_argument_at(1);

    let mut session = app::Session::new(); 

    session.run(action, argument);

    for res in session.action_responses.iter(){
        display_action_response(res);
    }
    
}

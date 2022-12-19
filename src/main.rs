mod app;
use brr::get_args;

fn main() {
    let mut args = get_args();
    let action = app::bash_driver::get_action(&args);
    let arguments = app::bash_driver::get_argument(&mut args).expect("E");

    let app = app::Session::new(); 
    app.run(action, arguments);
}

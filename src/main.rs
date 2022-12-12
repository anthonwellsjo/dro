mod app;
use brr::get_args;

fn main() {
    let args = get_args();
    let action = app::bash_driver::get_action(&args);
    let arguments = app::bash_driver::get_argument(&mut args);

    let mut app = app::Session::run(action);
    app.run();
}

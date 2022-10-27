mod app;

use app::utils::get_args;

fn main() {
    let args = get_arg(1);
    let mut app = app::Session::new(args);
    app.run();
}

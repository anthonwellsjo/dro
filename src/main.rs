mod app;

use app::utils::get_args;

fn main() {
    let args = get_args();
    let mut app = app::Session::new(args);
    app.run();
}

mod app;
use brr::get_args;

fn main() {
    let mut args = get_args();
    let action = app::bash_driver::get_action(&args);
    let arguments = app::bash_driver::get_argument(&mut args);

    let mut app = app::Session::new(); 
    let sprk = app.run(action, arguments);
    for a_r in sprk.iter(){
        println!("{:?}", a_r)
    }
    
}

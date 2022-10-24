mod db;
mod ui;
use cursive::{Cursive, CursiveExt};

pub struct App {
    pub ui: Cursive,
}

impl App {
    pub fn new() -> Self {
        App{
            ui: ui::create_app()
        }
    }
    pub fn run(mut self) {
        self.ui.run();
    }
}



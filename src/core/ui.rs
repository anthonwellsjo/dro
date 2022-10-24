use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ResizedView, SelectView,
};
use cursive::Cursive;

use super::db::{self, delete_todo_from_db, get_todos};

pub fn get_select_view(
    on_submit: fn(s: &mut Cursive, name: &str),
) -> ResizedView<NamedView<SelectView>> {
    SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5))
}

pub fn get_buttons(
    add_name: fn(s: &mut Cursive),
    delete_name: fn(s: &mut Cursive),
) -> LinearLayout {
    LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit))
}

pub fn compose_app_ui(
    s: &mut Cursive,
    select: ResizedView<NamedView<SelectView>>,
    buttons: LinearLayout,
) {
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Todo manager"),
    )
}

pub fn create_app() -> Cursive {
    let mut siv: Cursive = Cursive::default();
    let select = get_select_view(on_select);
    let buttons = get_buttons(add_todo, delete_todo);
    compose_app_ui(&mut siv, select, buttons);
    render_saved_todos(&mut siv);
    return siv;
}

fn on_select(s: &mut Cursive, desrciption: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("{}", desrciption))
            .title("Todo details")
            .button("Close", Cursive::quit),
    );
}

fn render_saved_todos(s: &mut Cursive) {
    s.call_on_name("select", |view: &mut SelectView<String>| {
        let todos =
            get_todos().unwrap_or_else(|err| panic!("Error while getting todos from db: {}", err));

        for todo in todos.iter() {
            view.add_item_str(&todo.description);
        }
    });
}

fn add_todo(s: &mut Cursive) {
    fn ok(s: &mut Cursive, description: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            db::save_todo_to_db(db::ToDo::new(description)).unwrap_or_else(|err| {
                panic!("Error while saving to_do: {}", err);
            });
            view.add_item_str(description);
        });
        s.pop_layer();
    }
    s.add_layer(
        Dialog::around(EditView::new().with_name("description").fixed_width(10))
            .title("Add todo")
            .button("OK", |s| {
                let description = s
                    .call_on_name("description", |view: &mut EditView| view.get_content())
                    .unwrap();
                ok(s, &description);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn delete_todo(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No todo to delete.")),
        Some(item_id) => {
            let selected_todo = select
                .get_item(item_id)
                .unwrap_or_else(|| panic!("Error while fetching todo that should e deleted."));
            delete_todo_from_db(selected_todo.0)
                .unwrap_or_else(|err| panic!("Error while deleting todo from db: {}", err));
            select.remove_item(item_id);
        }
    }
}

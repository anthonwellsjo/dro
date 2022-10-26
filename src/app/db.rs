use rusqlite::{Connection, Result};

static TEST_DB_PATH: &str = "./tests.sql";
static DB_PATH: &str = "./db.sql";

pub struct ToDo {
    pub description: String,
    pub done: bool,
}

impl ToDo {
    pub fn new(description: &str) -> ToDo {
        ToDo {
            description: description.to_owned(),
            done: false,
        }
    }
}

///  Gets connection to DB. This function will create a new DB if
///  not already present
pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS to_dos (
             id INTEGER PRIMARY KEY,
             description TEXT NOT NULL,
             done BOOL NOT NULL,
             created TEXT DEFAULT CURRENT_TIMESTAMP 
         )",
        [],
    )?;
    Ok(conn)
}

/// Gets all todos from the database
/// # Examples
/// ```
/// use core::db::get_todos;
/// let res = get_todos();
/// ```
pub fn get_todos() -> Result<Vec<ToDo>> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT description, done
         FROM to_dos",
    )?;

    let to_dos = stmt.query_map([], |row| {
        Ok(ToDo {
            description: row.get(0)?,
            done: row.get(1)?,
        })
    })?;

    let mut todiloes: Vec<ToDo> = Vec::new();

    for todo in to_dos {
        let greeting_file = match todo {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        todiloes.push(greeting_file);
    }

    Ok(todiloes)
}

/// Saves a todo to the database
/// # Arguments
/// * `to_do` - In instance of the ToDo struct that will be saved.
/// # Examples
/// ```
/// use core::db::{ToDo, save_todo_to_db};
/// let to_do = ToDo::new("Fix the bike wheel");
/// let res = save_todo_to_db(to_do);
/// assert_eq!(res, Ok(()));
/// ```
pub fn save_todo_to_db(to_do: ToDo) -> Result<ToDo> {
    let conn = get_db_connection()?;

    conn.execute(
        "INSERT INTO to_dos (description, done) values (?1, 0)",
        &[&to_do.description.to_string()],
    )?;

    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));

    Ok(to_do)
}

/// Deletes a todo to the database
/// # Arguments
/// * `to_do` - In instance of the ToDo struct that will be saved.
/// # Examples
/// ```
/// use core::db::{ToDo, save_todo_to_db, delete_todo_from_db};
/// let to_do = ToDo::new("Fix the bike wheel");
/// let res = save_todo_to_db(to_do);
/// delete_todo_from_db(&todo.description);
/// ```
pub fn delete_todo_from_db(description: &str) -> Result<()> {
    let conn = get_db_connection()?;
    conn.execute("DELETE FROM to_dos WHERE description=(?1)", &[&description])?;
    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));
    Ok(())
}

/// Gets db-path which depends on wether we are running a test or not.
fn get_db_path() -> &'static str {
    if cfg!(test) {
        &TEST_DB_PATH
    } else {
        &DB_PATH
    }
}

#[cfg(test)]
mod tests {

    use crate::app::db::delete_todo_from_db;

    use super::{get_todos, save_todo_to_db, ToDo, TEST_DB_PATH};
    use rand::Rng;
    use std::fs;

    #[test]
    fn save_a_todo() {
        let description = "Test description";
        let to_do = ToDo::new(description);
        let res = save_todo_to_db(to_do).unwrap();
        assert_eq!(&res.description, description);
    }

    #[test]
    fn save_and_load_todos_from_db() {
        let description = TestUtils::create_rnd_string();
        let description_two = TestUtils::create_rnd_string();
        let to_do = ToDo::new(&description);
        let to_do2 = ToDo::new(&description_two);
        save_todo_to_db(to_do).unwrap();
        save_todo_to_db(to_do2).unwrap();

        let todos = get_todos().unwrap();
        assert!(&todos.iter().any(|x| x.description == description_two));
    }

    #[test]
    fn save_and_delete_todos_from_db() {
        //Arrange: Create and save todos to db + assert
        let description = TestUtils::create_rnd_string();
        let to_do = ToDo::new(&description);
        save_todo_to_db(to_do).unwrap();
        let todos = get_todos().unwrap();
        let todo_from_db = &todos.iter().find(|i| i.description == description);
        assert!(&todos.iter().any(|x| x.description == description));
        assert_eq!(todo_from_db.is_some(), true);

        //Act: Delete todo
        let resp = delete_todo_from_db(&description);
        let todos = get_todos().unwrap();
        let todo_from_db = &todos.iter().find(|i| i.description == description);

        //Assert
        assert_eq!(resp, Ok(()));
        assert_eq!(todo_from_db.is_some(), false);
    }

    #[test]
    #[ignore]
    fn cleanup_test_database() {
        fn remove_test_db() {
            fs::remove_file(&TEST_DB_PATH)
                .unwrap_or_else(|err| panic!("Panicking while deleting test database: {}", err));
        }
        remove_test_db();
    }

    /// Contains common util functions and properties for testing
    struct TestUtils {}

    impl TestUtils {
        fn create_rnd_string() -> String {
            let mut rng = rand::thread_rng();
            let rand_num: u16 = rng.gen();
            rand_num.to_string()
        }
    }
}

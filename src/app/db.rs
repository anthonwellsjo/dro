use arw_brr::get_app_path;
use rusqlite::{Connection, Result};
use std::fs;

#[derive(Debug)]
pub struct Dro {
    pub description: String,
    pub done: bool,
}

impl Dro {
    pub fn new(description: &str) -> Dro {
        Dro {
            description: description.to_owned(),
            done: false,
        }
    }
}

///  Gets connection to DB. This function will create a new DB if
///  not already present
pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open(get_app_path("dro"))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS to_dos (
             id INTEGER PRIMARY KEY,
             description TEXT NOT NULL,
             done BOOL NOT NULL,
             created TEXT DEFAULT CURRENT_TIMESTAMP,
             deleted BOOL DEFAULT 0
         )",
        [],
    )?;
    Ok(conn)
}

/// Gets all dros from the database
/// # Examples
/// ```
/// use core::db::get_dros;
/// let res = get_dros();
/// ```
pub fn get_dros() -> Result<Vec<Dro>> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT description, done
         FROM to_dos 
         WHERE deleted=0",
    )?;

    let dros = stmt.query_map([], |row| {
        Ok(Dro {
            description: row.get(0)?,
            done: row.get(1)?,
        })
    })?;

    let mut todiloes: Vec<Dro> = Vec::new();

    for dro in dros {
        let greeting_file = match dro {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        todiloes.push(greeting_file);
    }

    Ok(todiloes)
}

/// Saves a dro to the database
/// # Arguments
/// * `to_do` - In instance of the dro struct that will be saved.
/// # Examples
/// ```
/// use core::db::{Dro, save_dro_to_db};
/// let to_do = Dro::new("Fix the bike wheel");
/// let res = save_dro_to_db(to_do);
/// assert_eq!(res, Ok(()));
/// ```
pub fn save_dro_to_db(to_do: &Dro) -> Result<()> {
    let conn = get_db_connection()?;

    conn.execute(
        "INSERT INTO to_dos (description, done) values (?1, 0)",
        &[&to_do.description.to_string()],
    )?;

    conn.close()
        .unwrap_or_else(|_| panic!("Panicking while closing conection."));

    Ok(())
}

/// Marks dro as done
/// # Arguments
/// * `description` - The description that matches dro that should get updated.
/// # Examples
/// ```
/// use core::db::{Dro, save_dro_to_db, delete_dro_from_db};
/// let to_do = Dro::new("Fix the bike wheel");
/// let res = save_dro_to_db(to_do);
/// mark_dro_as_done(&dro.description);
/// ```
pub fn mark_dro_as_done(description: &str) -> Result<()> {
    let conn = get_db_connection()?;
    conn.execute(
        "UPDATE to_dos SET done=1 WHERE description=(?1)",
        &[&description],
    )?;
    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));
    Ok(())
}

/// Marks dro as undone
/// # Arguments
/// * `description` - The description that matches dro that should get updated.
/// # Examples
/// ```
/// use core::db::{Dro, save_dro_to_db, delete_dro_from_db};
/// let to_do = Dro::new("Fix the bike wheel");
/// let res = save_dro_to_db(to_do);
/// mark_dro_as_done(&dro.description);
/// mark_dro_as_undone(&dro.description);
/// ```
pub fn mark_dro_as_undone(description: &str) -> Result<()> {
    let conn = get_db_connection()?;
    conn.execute(
        "UPDATE to_dos SET done=0 WHERE description=(?1)",
        &[&description],
    )?;
    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));
    Ok(())
}

/// Marks all done dros as deleted - not destructive
/// # Arguments
/// * `description` - The description that matches dro that should get updated.
/// # Examples
/// ```
/// use core::db::{Dro, save_dro_to_db, delete_dro_from_db};
/// let to_do = Dro::new("Fix the bike wheel");
/// let res = save_dro_to_db(to_do);
/// mark_dro_as_done(&dro.description);
/// purge_dros():
/// ```
pub fn purge_dros() -> Result<()> {
    let conn = get_db_connection()?;
    conn.execute("UPDATE to_dos SET deleted=1 WHERE done=1", [])?;
    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_dros, mark_dro_as_done, save_dro_to_db, Dro};
    use crate::app::db::{mark_dro_as_undone};
    use arw_brr::get_app_path;
    use rand::Rng;
    use std::fs;

    #[test]
    fn grab_dros() {
        cleanup_test_database();
        let descs = vec!["one", "two", "three"];
        for desc in descs.iter() {
            let to_do = Dro::new(desc);
            save_dro_to_db(&to_do).unwrap();
        }
        let dros_from_db = get_dros().unwrap();
        let mut descs_from_db = dros_from_db
            .iter()
            .map(|dro| -> &str { &dro.description });
        assert!(descs_from_db.all(|item| descs.contains(&item)));
    }

    #[test]
    fn save_a_dro() {
        let description = "Test description";
        let to_do = Dro::new(description);
        save_dro_to_db(&to_do).unwrap();
        let to_dos = get_dros().unwrap();
        assert_eq!(to_dos.iter().any(|i| i.description == description), true);
    }

    #[test]
    fn save_and_load_dros_from_db() {
        let description = TestUtils::create_rnd_string();
        let description_two = TestUtils::create_rnd_string();
        let to_do = Dro::new(&description);
        let to_do2 = Dro::new(&description_two);
        save_dro_to_db(&to_do).unwrap();
        save_dro_to_db(&to_do2).unwrap();

        let dros = get_dros().unwrap();
        assert!(&dros.iter().any(|x| x.description == description_two));
    }

    #[test]
    fn mark_as_done() {
        cleanup_test_database();
        let description = TestUtils::create_rnd_string();
        let to_do = Dro::new(&description);
        save_dro_to_db(&to_do).unwrap();
        mark_dro_as_done(&description).unwrap();
        let dros = get_dros().unwrap();
        let dro: &Dro = dros.iter().nth(0).unwrap();
        assert_eq!(dro.done, true);
    }

    #[test]
    fn mark_as_undone() {
        cleanup_test_database();
        let description = TestUtils::create_rnd_string();
        let to_do = Dro::new(&description);
        save_dro_to_db(&to_do).unwrap();
        mark_dro_as_done(&description).unwrap();
        let dros_done = get_dros().unwrap();
        let dro_done: &Dro = dros_done.iter().nth(0).unwrap();
        assert_eq!(dro_done.done, true);
        mark_dro_as_undone(&description).unwrap();
        let dros_undone = get_dros().unwrap();
        let dro_undone: &Dro = dros_undone.iter().nth(0).unwrap();
        assert_eq!(dro_undone.done, false);
    }

    #[test]
    #[ignore]
    fn cleanup_test_database() {
        fn remove_test_db() {
            fs::remove_file(get_app_path("dro"))
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

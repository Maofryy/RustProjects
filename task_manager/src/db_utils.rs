use crate::tasks::Task;
use rusqlite::{params, Connection, Error, Result};

pub struct SQLHelper;

impl SQLHelper {
    pub fn create_db(filename: &str) -> Result<()> {
        let conn = Connection::open(filename)?;

        conn.execute(
            "create table if not exists tasks (
                id integer primary key,
                name text not null,
                status integer
            )",
            [],
        )?;

        Ok(())
    }

    pub fn insert_task(filename: &str, task: Task) -> Result<()> {
        let conn = Connection::open(filename)?;

        conn.execute(
            "INSERT INTO tasks (name, status) values (?1, ?2)",
            params![&task.label.to_string(), &task.status],
        )?;

        Ok(())
    }

    pub fn read_task_by_id(filename: &str, id: u64) -> Result<Task, Error> {
        let conn = Connection::open(filename)?;

        let mut stmt = conn.prepare("SELECT id, name, status FROM tasks WHERE id=?1")?;
        let task_row = stmt.query_row([&id], |row| {
            Ok(Task {
                id: row.get(0)?,
                label: row.get(1)?,
                status: row.get(2)?,
            })
        })?;
        return Ok(task_row);
    }

    pub fn print_tasks(filename: &str) -> Result<()> {
        let conn = Connection::open(filename)?;

        let mut stmt = conn.prepare("SELECT id, name, status FROM tasks")?;
        let tasks_collection = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                label: row.get(1)?,
                status: row.get(2)?,
            })
        })?;
        // Print all tasks
        for task in tasks_collection {
            println!("{:?}", task.unwrap())
        }
        Ok(())
    }

    pub fn delete_task(filename: &str, id: u64) -> Result<()> {
        let conn = Connection::open(filename)?;

        conn.execute(
            "DELETE FROM tasks
            WHERE id = ?1;",
            params![&id],
        )?;

        Ok(())
    }

    pub fn clear_tasks(filename: &str) -> Result<()> {
        let conn = Connection::open(filename)?;

        conn.execute("DELETE FROM tasks", [])?;

        Ok(())
    }

    pub fn update_status(filename: &str, id: u64, status: bool) -> Result<()> {
        let conn = Connection::open(filename)?;

        conn.execute(
            "UPDATE tasks
            SET status = ?1
            WHERE id = ?2;",
            params![&status, &id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SQLHelper;
    use crate::tasks::Task;
    use std::fs;

    #[test]
    fn test_create_db() {
        let db_path = "test_create.db";
        assert!(SQLHelper::create_db(db_path).is_ok());
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_insert_db() {
        let db_path = "test_insert.db";
        SQLHelper::create_db(db_path).unwrap();
        assert!(SQLHelper::insert_task(db_path, Task::new(0, "test1".to_string(), false)).is_ok());
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_print_tasks() {
        let db_path = "test_print.db";
        SQLHelper::create_db(db_path).unwrap();
        // print and assert is_ok()
        assert!(SQLHelper::print_tasks(db_path).is_ok());
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_delete_task() {
        // test deleting non existing, assert is_error()
        let db_path = "test_del.db";
        SQLHelper::create_db(db_path).unwrap();
        // insert and assert true on delete
        SQLHelper::insert_task(db_path, Task::new(1, "test1".to_string(), false)).unwrap();
        assert!(SQLHelper::delete_task(db_path, 1).is_ok());
        assert!(SQLHelper::read_task_by_id(db_path, 1).is_err());

        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_read_task() {
        let db_path = "test_read_db";
        SQLHelper::create_db(db_path).unwrap();
        let test_task = Task::new(1, "reading test".to_string(), false);
        SQLHelper::insert_task(db_path, test_task).unwrap();
        assert_eq!(
            SQLHelper::read_task_by_id(db_path, 1).unwrap(),
            Task::new(1, "reading test".to_string(), false)
        );
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_clear_tasks() {
        let db_path = "test_clear.db";
        SQLHelper::create_db(db_path).unwrap();
        // assert is_ok()
        assert!(SQLHelper::clear_tasks(db_path).is_ok());
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_update_test() {
        let db_path = "test_update.db";
        SQLHelper::create_db(db_path).unwrap();
        // update non existing, assert is_error()
        // insert and update, assert is_ok()
        SQLHelper::insert_task(db_path, Task::new(1, "test1".to_string(), false)).unwrap();
        assert!(SQLHelper::update_status(db_path, 1, true).is_ok());
        if let Ok(ret_task) = SQLHelper::read_task_by_id(db_path, 1) {
            assert_eq!(ret_task.status, true);
        }
        fs::remove_file(db_path).unwrap();
    }
}

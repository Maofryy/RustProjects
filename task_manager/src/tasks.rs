use crate::db_utils::SQLHelper;
use rusqlite::Result;

#[derive(PartialEq)]
pub struct Task {
    pub id: u64,
    pub label: String,
    pub status: bool,
}

impl std::fmt::Debug for Task {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let status_char;
        if self.status {
            status_char = "x";
        } else {
            status_char = " ";
        }
        write!(fmt, "{} [{}] {}", self.id, status_char, self.label)
    }
}

impl Task {
    pub fn new(id: u64, label: String, status: bool) -> Task {
        Task {
            id: id,
            label: label,
            status,
        }
    }

    pub fn update(&mut self, status: bool) {
        self.status = status;
    }
}

pub struct Command {
    pub command: String,
    pub args: String,
}

impl Command {
    pub fn new(cmd: String, args: String) -> Command {
        Command {
            command: cmd,
            args: args,
        }
    }

    fn print_result_msg(res: Result<()>, msg: &str) {
        match res {
            Ok(()) => println!("{}", msg),
            Err(e) => println!("{}", e),
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret: String = self.command.to_owned();
        ret = ret + " | " + &self.args;
        return ret;
    }

    pub fn check_command(&self) -> bool {
        // let commands: Vec<&str> = Vec::new();
        let commands = vec!["+", "-", "x", "c", "clear"];
        if commands.contains(&&*self.command) {
            return true;
        } else {
            return false;
        }
    }

    // Print tasks
    fn print_tasks(db_path: &str) {
        match SQLHelper::print_tasks(db_path) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    }

    // Add task
    fn add_task(db_path: &str, args: String) {
        match SQLHelper::insert_task(db_path, Task::new(0, args.to_owned(), false)) {
            Ok(()) => println!("Inserted task {}.", &args),
            Err(e) => println!("{}", e),
        }
    }

    // Delete task
    fn delete_task(db_path: &str, args: String) {
        let key: u64 = match args.parse::<u64>() {
            Ok(number) => number,
            Err(e) => {
                println!("{}", e.to_string());
                0
            }
        };
        match SQLHelper::delete_task(db_path, key) {
            Ok(()) => println!("Deleted task {}.", key),
            Err(e) => println!("{}", e),
        };
    }

    // Update task
    fn update_task(db_path: &str, args: String, status: bool) {
        let key: u64 = match args.parse::<u64>() {
            Ok(number) => number,
            Err(e) => {
                println!("{}", e.to_string());
                0
            }
        };
        match SQLHelper::update_status(db_path, key, status) {
            Ok(()) => println!("Updated task {}.", key),
            Err(e) => println!("{}", e),
        };
    }

    // Delete all tasks
    fn clear_tasks(db_path: &str) {
        match SQLHelper::clear_tasks(db_path) {
            Ok(()) => println!("Cleared all tasks."),
            Err(e) => println!("{}", e),
        };
    }

    //Handle Command
    pub fn handle_command(&self, db_path: &str) {
        if self.command.eq("+") {
            Command::add_task(db_path, self.args.to_owned());
        } else if self.command.eq("-") {
            Command::delete_task(db_path, self.args.to_owned());
        } else if self.command.eq("x") {
            Command::update_task(db_path, self.args.to_owned(), true);
        } else if self.command.eq("c") {
            Command::update_task(db_path, self.args.to_owned(), false);
        } else if self.command.eq("clear") {
            Command::clear_tasks(db_path);
        } else {
            // Basic behavior:
            println!("{}", self.to_string())
        }
        Command::print_tasks(db_path);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_command() {
        assert!(Command::new("+".to_string(), "test".to_string()).check_command());
        assert!(Command::new("-".to_string(), "test".to_string()).check_command());
        assert!(Command::new("x".to_string(), "test".to_string()).check_command());
        assert!(Command::new("c".to_string(), "test".to_string()).check_command());
        assert!(Command::new("clear".to_string(), "test".to_string()).check_command());
        assert!(!Command::new("*".to_string(), "test".to_string()).check_command());
        assert!(!Command::new("quit".to_string(), "test".to_string()).check_command());
    }
}

#![allow(dead_code, unused_variables)]
use std::io::{self, Write};
mod db_utils;
mod tasks;

// Command line struct

fn print_usage() {
    println!("Usage : ");
    println!("\t+ <label>   : Create new task with <label>");
    println!("\t- <id>      : Remove task from list by <id>");
    println!("\tx <id>      : Check task by <id>");
    println!("\tc <id>      : Uncheck task by <id>");
    println!("\tclear       : Erase all tasks");
    println!("\tquit        : Exit task manager\n");
}

fn main() {
    // User can entre multiple commands:
    //  + [label]
    //  - [number]
    //  x [number]
    //  c [number]
    //  quit
    //  clear

    println!("Hello and welcome to your Rust task manager!");
    print_usage();

    // init structures, maybe deserialize some of them
    // Infinite Loop
    //      get user input
    //      handle wrong commands
    //      execute commands
    //      ?serialize

    // connect to db
    let db_path = "taskmanager.db";

    //  if file doesnt exists : create tables, or include it in tables ?
    // Rewrite commands to not interface with a Hashmap attribut but directly with the db
    db_utils::SQLHelper::print_tasks(db_path).unwrap();
    io::stdout().flush().unwrap();
    let mut looping: bool = true;
    while looping {
        // Query new line
        print!(">>> ");
        io::stdout().flush().unwrap();

        // get user input
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error: Wrong input.");
        if input.eq("\r\n") {
            continue;
        }
        let line_vec: Vec<&str> = input.split_whitespace().collect();
        let cmd = tasks::Command::new(line_vec[0].to_string(), line_vec[1..].join(" ").to_string());
        if cmd.command.eq("quit") {
            println!("Terminating program.");
            looping = false;
        } else if cmd.check_command() {
            // Execute commands
            cmd.handle_command(db_path);
        } else {
            // Handle wrong commands
            println!("Error: wrong command, please refer to usage");
            print_usage();
        }
        // println!("command: {} | args= {}", cmd.command, cmd.args);
        // Serialize
    }
}

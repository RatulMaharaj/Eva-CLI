mod folders;
mod init;
mod search;
mod update;

use folders::{add_folder, remove_folder};
use init::create_tables;
use search::search;
use std::env::args;
use update::update;

fn main() {
    // Create database and tables if they don't exist
    create_tables().ok();

    // listen for command from CLI
    let args: Vec<String> = args().collect();

    if &args.len() == &1 {
        println!("Please provide a valid action to be performed e.g. search, add or update.");
    } else {
        // grab first arg
        let action = String::from(&args[1]);
        let mut parameter: String = String::from("");

        // grab second arg if provided
        if &args.len() > &2 {
            parameter = String::from(&args[2]);
        }

        match action.as_str() {
            "search" => {
                if &parameter == "" {
                    println!("Please provide a search parameter.")
                } else {
                    search(&parameter);
                }
            }
            "add" => {
                if &parameter == "" {
                    println!("Please provide a dir to add.")
                } else {
                    add_folder(&parameter);
                }
            }
            "remove" => {
                if &parameter == "" {
                    println!("Please provide a dir to remove.")
                } else {
                    remove_folder(&parameter);
                }
            }
            "update" => {
                update();
            }
            _ => {
                println!("Please provide a valid action.");
            }
        };
    }

    // println!("In file {}", parameter);
}

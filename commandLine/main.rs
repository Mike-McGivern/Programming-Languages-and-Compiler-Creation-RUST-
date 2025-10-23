use std::env;
use std::fs;
//use std::io;
use std::process;

//Programed by Michael McGivern
fn main() {
        let args: Vec<String> = env::args().collect();
        //creates a vector of strings from the arguments given at the command line

    if args.len() < 2 { //checks if arguments are given
        print_general_help();
        process::exit(0);
    }

    let command = &args[1];

    match command.as_str() {
        "help" => {
            if args.len() > 2 { //asking for help for certain command
                print_command_help(&args[2]);
            }
            else { //wants general help
                print_general_help();

            }
        }
        "print" => {
            handle_print_command(&args[2..])
            //prints arguments given from index 2 upto length of arguments
        }
        "list" => {
            list_commands();
        }
        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' for a list of commands.");
            process::exit(0);
        }
    }
}

fn print_general_help() {
    println!("A command line utility for Rust");
    println!("Usage: cargo run -- <command> [arguments]");
    println!();
    println!("Commands:");
    println!("    help        Print this help message");
    println!("    help [command]       shows help information for a command");
    println!("    print <file> [numbered]       Print arguments given");
    println!("    list        List all commands");
}

fn print_command_help(command:&str) {
    match command {
        "help" => {
            println!("help - Show help information");
            println!();
            println!("Usage: ");
            println!("cargo run -- help [command]");
            println!();
            println!("Description: ");
            println!("    Prints help information for a command");
            println!("Arguments: ");
            println!("    [command] - The command to get help for (OPTIONAL)");
        }

        "print" => {
            println!("print file contents");
            println!();
            println!("Usage: ");
            println!("cargo run -- print <file> [numbered]");
            println!();
            println!("Description: ");
            println!("prints the contents of the specified file");
            println!("Arguments: ");
            println!("    <file> - The path of the file to print (REQUIRED)");
            println!("    [numbered] - Whether to number the lines (OPTIONAL)");
        }

        "list" => {
            println!("list - List all commands");
            println!();
            println!("Usage: ");
            println!("cargo run -- list");
            println!();
            println!("Description: ");
            println!("lists all commands");
        }

        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' to learn how to use this tool or list for a list of commands.");
            process::exit(1);
        }
    }
}

fn handle_print_command(args: &[String]) {
    if args.is_empty() {
        println!("No file specified");
        println!("USEAGE: cargo run -- print <file> [--numbered]");
        println!("Try 'help print' for more information");
        process::exit(0);
    }

    let file_path:&String = &args[0];
    let numbered:bool = args.len() > 1 && args[1] == "--numbered";

    match fs::read_to_string(file_path) {
        Ok(contents)  => {
            if numbered {
                for(line_number, line) in contents.lines().enumerate() {
                    println!("{:5} {}", line_number + 1, line);
                }
            } else {
                println!("{}", contents);
            }
        }
        Err(error) => {
            println!("Error reading file {}: {}", file_path, error);
            process::exit(0);
        }
    }
}

fn list_commands() {
    println!("Available commands:");
    println!("list - List all commands");
    println!("help - Show help information");
    println!("print - Print arguments given");
}

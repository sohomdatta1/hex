use pager::Pager;
use std::env;
use std::io;
use std::process;

mod chrome;
mod envman;
mod hexdump;
mod print_error;
mod strings;
mod search;

static ERROR_EXIT_CODE: i32 = -1;
static PROG_NAME: &str = "hex";

fn help() {
    println!("Usage: {} <operation> [filename] [...options]", PROG_NAME);
    println!("Operations:");
    println!(" help - you're looking at it");
    println!(" version - print version");
    println!(" str <file> - extract all strings in the file");
    println!(" dump <file> - hexdump the file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help();
        process::exit(ERROR_EXIT_CODE);
    }

    let operation = &args[1];
    match operation.as_str() {
        "help" => help(),
        "version" => println!("0.0.0"),
        "str" => {
            if args.len() < 3 {
                println!("Usage: {} str <filename>", PROG_NAME);
                process::exit(ERROR_EXIT_CODE);
            }

            let should_have_color_support = chrome::should_have_color_support();

            Pager::with_pager("less -R").setup();

            match strings::main_strings(&args[2], should_have_color_support) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "dump" => {
            if args.len() < 3 {
                println!("Usage: {} dump <filename> <search_sequence>", PROG_NAME);
                process::exit(ERROR_EXIT_CODE);
            }

            let should_have_color_support = chrome::should_have_color_support();

            if termion::is_tty(&mut io::stdout()) {
                envman::set_env("LESS", "-Ps| -offset- \\: 0 1  2 3  4 5  6 7  8 9  A B  C D  E F  | 0123456789ABCDEF |");
                // now that's what I call a hack :)
            }

            let mut pager = Pager::with_pager("less -R");
            pager.setup();

            match hexdump::main_hexdump(&args[2], should_have_color_support) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        "search" => {
            if args.len() < 5 {
                println!("Usage {} search <filename> <--binary|--str|--hex> <searchterm>", PROG_NAME);
                process::exit(ERROR_EXIT_CODE);
            }

            let should_have_color_support = chrome::should_have_color_support();

            let mut pager = Pager::with_pager("less -R");
            pager.setup();

            let str_as_bytes;
            if args[3].eq( "--str" ) {
                str_as_bytes = args[4].clone().into_bytes()
            } else {
                println!("Unimplemented");
                process::exit(ERROR_EXIT_CODE);
            }

            match search::main_search(&args[2], &str_as_bytes, should_have_color_support) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(ERROR_EXIT_CODE);
                }
            }
        }
        _ => {
            println!("Unknown operation: {}", operation);
            help();
            process::exit(ERROR_EXIT_CODE);
        }
    }
}

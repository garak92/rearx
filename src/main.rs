use std::io::stdin;
#[macro_use]
extern crate prettytable;
use request::{deserialize_json, read_yaml, Content, RequestData};
use std::process::Command;
use structopt::StructOpt;
use table::create_table;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod request;
mod table;

fn main() {
    let host = read_yaml();
    search(1, host); //Initialize search on page number 1
}
fn search(mut num: i32, host: String) {
    println!("Retrieving page {}...", num);
    let args = Arguments::from_args(); //Gets the command line arguments from Arguments struct
    match deserialize_json(args.query, num) {
        Ok(data) => {
            println!("{}", termion::clear::BeforeCursor); //Clears like this are only there for aesthetics
            let contents = create_table(data as RequestData);
            println!("HOST: {}", host);
            println!("PAGE: {}", num);
            let stdin = stdin();
            let terminal = std::io::stdout().into_raw_mode().unwrap();
            terminal.activate_raw_mode().unwrap();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Right => {
                        terminal.suspend_raw_mode().unwrap(); //Necessary for output not to be scrambled
                        num += 1;
                        println!("{}", termion::clear::All);
                        search(num, host);
                        break;
                    }
                    Key::Left => {
                        terminal.suspend_raw_mode().unwrap();
                        num -= 1;
                        println!("{}", termion::clear::All);
                        println!("Retrieving page {}...", num);
                        search(num, host);
                        break;
                    }
                    Key::Char('0') => {
                        open_result(0, &contents);
                    }
                    Key::Char('1') => {
                        open_result(1, &contents);
                    }
                    Key::Char('2') => {
                        open_result(2, &contents);
                    }
                    Key::Char('3') => {
                        open_result(3, &contents);
                    }
                    Key::Char('4') => {
                        open_result(4, &contents);
                    }
                    Key::Char('5') => {
                        open_result(5, &contents);
                    }
                    Key::Char('6') => {
                        open_result(6, &contents);
                    }
                    Key::Char('7') => {
                        open_result(7, &contents);
                    }
                    Key::Char('8') => {
                        open_result(8, &contents);
                    }
                    Key::Char('9') => {
                        open_result(9, &contents);
                    }
                    Key::Char('f') => {
                        terminal.suspend_raw_mode().unwrap();
                        num = 1;
                        println!("{}", termion::clear::All);
                        println!("Returning to first page...");
                        search(num, host);
                        break;
                    }

                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

#[derive(StructOpt)]
#[structopt(
    about = "Welcome to Rearx, a TUI client for the Searx meta-search engine, written in Rust!"
)]
struct Arguments {
    query: String,
}

fn open_result(num: usize, contents: &Vec<Content>) {
    Command::new("xdg-open")
        .arg(&contents[num].url)
        .spawn()
        .expect("failed to execute process");
}

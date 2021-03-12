#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    match deserialize_json() {
        Ok(data) => {
            create_table(data as RequestData);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

#[tokio::main]
async fn deserialize_json() -> Result<RequestData, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://searx.garudalinux.org/search?q=karl%20poppers&categories=general&format=json&lang=en&page=1")
        .await?
        .json::<RequestData>()
        .await?;
    //println!("{:#?}", resp.query);
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestData {
    query: String,
    number_of_results: f32,
    results: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    title: String,
    engine: String,
    content: String,
    pretty_url: String,
}

fn create_table(data: RequestData) {
    let contents: Vec<Content> = data.results;
    let lenght = contents.len();
    let mut table = Table::new();
    table.set_titles(row![b->"No", b->"Title", b->"Search Engine", b->"URL"]);

    for i in 0..lenght {
        table.add_row(row![
            i,
            contents[i].title,
            contents[i].engine,
            contents[i].pretty_url
        ]);
    }
    table.printstd();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => {
                Command::new("sh")
                    .arg("xdg-open")
                    .arg(&contents[0].pretty_url)
                    .output()
                    .expect("failed to execute process");
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

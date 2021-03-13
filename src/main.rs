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
            let contents = create_table(data as RequestData);
            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode().unwrap();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Char('0') => {
                        open_result(0, &contents);
                    }
                    _ => {}
                }
                stdout.flush().unwrap();
            }
        }
        Err(e) => {
            println!("Error: {}", e);
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

    fn open_result(num: usize, contents: &Vec<Content>) {
        Command::new("sh")
            .arg("xdg-open")
            .arg(&contents[num].pretty_url)
            .output()
            .expect("failed to execute process");
    }

    fn create_table(data: RequestData) -> Vec<Content> {
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

        return contents;
    }
}

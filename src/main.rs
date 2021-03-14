#[macro_use]
extern crate prettytable;
use prettytable::Table;
use serde::{Deserialize, Serialize};
use std::io::stdin;
use std::process::Command;
use structopt::StructOpt;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    search(1);
}
fn search(num: i32) {
    println!("{}", termion::clear::All);
    let args = Arguments::from_args();
    match deserialize_json(args.query, num) {
        Ok(data) => {
            let contents = create_table(data as RequestData);
            println!("Page: {}", num);
            let stdin = stdin();
            let terminal = std::io::stdout().into_raw_mode().unwrap();
            terminal.activate_raw_mode().unwrap();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => break,
                    Key::Right => {
                        terminal.suspend_raw_mode().unwrap();
                        println!("{}", termion::clear::All);
                        search(num + 1);
                        break;
                    }
                    Key::Left => {
                        terminal.suspend_raw_mode().unwrap();
                        println!("{}", termion::clear::All);
                        search(num - 1);
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

                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    #[tokio::main]
    async fn deserialize_json(
        query: String,
        num: i32,
    ) -> Result<RequestData, Box<dyn std::error::Error>> {
        let page_num = num.to_string();
        let server = String::from("https://searx.garudalinux.org/search?q=");
        let arguments = String::from("&categories=general&format=json&lang=en&pageno=");
        let request = [server, query, arguments, page_num].concat();
        let resp = reqwest::get(request).await?.json::<RequestData>().await?;
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

    #[derive(StructOpt)]
    struct Arguments {
        query: String,
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
        if lenght <= 9 {
            for i in 0..lenght {
                table.add_row(row![
                    i,
                    contents[i].title,
                    contents[i].engine,
                    contents[i].pretty_url
                ]);
            }
        } else {
            for i in 0..9 {
                table.add_row(row![
                    i,
                    contents[i].title,
                    contents[i].engine,
                    contents[i].pretty_url
                ]);
            }
        }
        table.printstd();
        println!(
            "Query: {}\nSearch results: {}",
            data.query, data.number_of_results
        );

        return contents;
    }
}

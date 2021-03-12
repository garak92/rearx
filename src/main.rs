use serde::{Deserialize, Serialize};
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

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
    let resp = reqwest::get("https://searx.garudalinux.org/search?q=solar%20panels&categories=general&format=json&lang=en&page=1")
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
    table.set_titles(row![b->"Title", b->"Search Engine", b->"Content", b->"URL"]);
    for i in 0..lenght {
        table.add_row(row![
            contents[i].title,
            contents[i].engine,
            contents[i].content,
            contents[i].pretty_url
        ]);
    }
    table.printstd();
}

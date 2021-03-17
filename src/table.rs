use crate::request::{Content, RequestData};
use prettytable::{format, Table};
//This module takes data from requests.rs and builds a table with it

pub fn create_table(data: RequestData) -> Vec<Content> {
    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders('|')
        .padding(1, 1)
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .build();
    let contents: Vec<Content> = data.results; //Get search results from struct
    let lenght = contents.len();
    let mut table = Table::new();
    if lenght <= 9 {
        for i in 0..lenght {
            table.add_row(row![bBbcFd->i]);
            table.add_row(row![bBbcFd->contents[i].title]);
            table.add_row(row![bBbcFd->contents[i].engine]);
            table.add_row(row![Fbc->contents[i].url]);
        }
    } else {
        for i in 0..9 {
            table.add_row(row![bBbcFd->i]);
            table.add_row(row![bBbcFd->contents[i].title]);
            table.add_row(row![bBbcFd->contents[i].engine]);
            table.add_row(row![Fbc->contents[i].url]);
        }
    }
    table.set_format(format); //Applies format
    table.printstd();
    println!(
        "QUERY: {}\nSEARCH RESULTS: {}",
        data.query, data.number_of_results
    );

    return contents; //This makes it possible to reference result number and url on main.rs
}

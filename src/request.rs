use serde::{Deserialize, Serialize};
//This module sends a request to a Searx instance, gets a json response, and deserializes it into two structs

#[tokio::main]
pub async fn deserialize_json(
    query: String,
    num: i32,
) -> Result<RequestData, Box<dyn std::error::Error>> {
    let page_num = num.to_string(); //Convert page number to string in order to concat with the other params
    let server = String::from("https://searx.garudalinux.org/search?q=");
    let arguments = String::from("&categories=general&format=json&lang=en&pageno=");
    let request = [server, query, arguments, page_num].concat();
    let resp = reqwest::get(request).await?.json::<RequestData>().await?;
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub query: String,
    pub number_of_results: f32,
    pub results: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub title: String,
    pub engine: String,
    pub content: String,
    pub pretty_url: String,
}

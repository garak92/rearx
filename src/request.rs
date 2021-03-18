use reqwest::get;
use serde::{Deserialize, Serialize};
//This module sends a request to a Searx instance, gets a json response, and deserializes it into two structs

#[tokio::main]
pub async fn deserialize_json(
    query: String,
    num: i32,
) -> Result<RequestData, Box<dyn std::error::Error>> {
    let page_num = num.to_string(); //Convert page number to string in order to concat with the other params
    let mut server = read_yaml();
    server.push_str("search?q=");
    let arguments = String::from("&categories=general&format=json&lang=en&pageno=");
    let request = [server, query, arguments, page_num].concat();
    let resp = get(&request)
        .await?
        .json::<RequestData>()
        .await
        .expect(&get(&request).await?.text().await? as &str);
    Ok(resp)
}

pub fn read_yaml() -> String {
    let path = "/etc/rearx/rearx.yaml";
    let f = std::fs::File::open(path).expect(
        "Failed to read config file on /etc/rearx/rearx.yaml. Perhaps you need to create it.",
    );
    let d: String = serde_yaml::from_reader(f).expect(
        "Could not parse rearx.yaml, perhaps bad syntax or you didn't specify any Searx instance",
    );
    return d;
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
    pub url: String,
}

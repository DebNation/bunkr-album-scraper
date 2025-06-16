use regex::Regex;
use reqwest::{self, Client};
use tokio::io::{self, AsyncBufReadExt, BufReader};
mod utils;

#[tokio::main]
async fn main() {
    let mut url = String::from("");
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    println!("Enter Url: ");
    if let Ok(Some(line)) = reader.next_line().await {
        url = line.trim().to_string();
    }

    let re = Regex::new(r#"href="(?P<href>/f/[^"]+)""#).unwrap();

    let body_str: String = get_response_body(&url).await;

    let mut url_vec: Vec<String> = Vec::new();

    let base_url = url.split("/a/").next().unwrap_or("failed to get base url");

    for caps in re.captures_iter(&body_str) {
        let x = format!("{}{}", base_url, &caps["href"]);
        url_vec.push(x.to_string());
    }
    let mut direct_urls_vec: Vec<String> = Vec::new();
    for url in &url_vec {
        let direct_url = get_direct_url(url).await;
        direct_urls_vec.push(direct_url);
    }
    // println!("{:?}", direct_urls_vec);
    for url in &direct_urls_vec {
        let _ = utils::download::download_link(url).await.unwrap();
        println!("Downloaded {}", url);
    }
}

async fn get_response_body(url: &str) -> String {
    let client = Client::new();
    let res = client
        .post(url)
        .header("Content-Length", "0")
        .send()
        .await
        .unwrap();

    let body = res.text().await.unwrap();

    let body_str = format!("{}", body);
    return body_str;
}

async fn get_direct_url(url: &str) -> String {
    let body_str: String = get_response_body(url).await;
    let re = Regex::new(r#"https://get\.bunkrr\.su/file/[^"]*"#).unwrap();
    if let Some(caps) = re.captures(&body_str) {
        let direct_url = caps.get(0).unwrap().as_str();
        return direct_url.to_string();
    } else {
        return "".to_string();
    }
}

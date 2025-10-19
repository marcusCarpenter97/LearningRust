// Program that downloads the contents of a web page given a URL.
use std::env;
use url::Url;
use reqwest;
use std::error::Error;

fn is_valid_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}

async fn get_url_content(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    // Get all command-line arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Enter a URL when calling the programme.");
    }

    if args.len() > 1 {
        let url = &args[1];
        if is_valid_url(url) {
            println!("Downloading content from {}", url);
            match get_url_content(url).await {
                Ok(contents) => println!("{}", contents),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

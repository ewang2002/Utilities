use std::io;
use std::env;
use std::error::Error;
use serde_json::Value;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Try to get BITLY_API environmental variable.
    let bitly_api_key = match env::var("BITLY_API") {
        Ok(r) => r,
        Err(_) => {
            println!("No \"BITLY_API\" environmental variable defined.");
            exit(1);
        }
    };

    // 1st argument (idx 0) will be the executable name.
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut full_str = if args.is_empty() { String::new() } else { args.join(" ") };
    if args.is_empty() {
        println!("You did not provide any command line arguments. Please type your query here.");
        io::stdin().read_line(&mut full_str)?;
    }

    let url_to_bitly = format!("https://letmegooglethat.com/?q={}", encode_string(full_str.as_str()));
    let client = reqwest::Client::new();

    let mut json_map: HashMap<&str, &str> = HashMap::new();
    json_map.insert("long_url", url_to_bitly.as_str());
    let res = client.post("https://api-ssl.bitly.com/v4/shorten")
        .json(&json_map)
        .header(AUTHORIZATION, format!("Bearer {}", bitly_api_key))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let status_code = res.status();
    println!("LMGTFY Link: {}", url_to_bitly);

    let json: Value = res.json().await?;
    if !status_code.is_success() {
        println!("Bit.ly: N/A (Failed w/ Status Code: {}).", status_code);
        println!("\tError: {}", json.to_string());
        return Ok(());
    }

    let link = json["link"].as_str();
    match link {
        Some(s) => println!("Bit.ly: {}", s),
        None => println!("Bit.ly: N/A (Error)")
    };

    return Ok(());
}

/// Encodes a string to its URL-encoded format. These rules are based on
/// [this](https://www.w3schools.com/tags/ref_urlencode.ASP) website.
///
/// # Parameters
/// - `s`: The string.
///
/// # Returns
/// The formatted string.
#[inline]
fn encode_string(s: &str) -> String {
    return s
        .replace('%', "%25")
        .replace(' ', "%20")
        .replace('!', "%21")
        .replace('"', "%22")
        .replace('#', "%23")
        .replace('$', "%24")
        .replace('&', "%26")
        .replace("'", "%27")
        .replace('(', "%28")
        .replace(')', "%29")
        .replace('*', "%2A")
        .replace('+', "%2B")
        .replace(',', "%2C")
        .replace('-', "%2D")
        .replace('.', "%2E")
        .replace('/', "%2F")
        .replace(':', "%3A")
        .replace(';', "%3B")
        .replace('<', "%3C")
        .replace('=', "%3D")
        .replace('>', "%3E")
        .replace('?', "%3F")
        .replace('@', "%40")
        .replace('[', "%5B")
        .replace('\\', "%5C")
        .replace(']', "%5D")
        .replace('^', "%5E")
        .replace('_', "%5F")
        .replace('`', "%60")
        .trim()
        .to_string();
}
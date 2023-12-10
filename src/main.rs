mod logger;

use std::fs;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use reqwest::StatusCode;

const URL: &str = "https://xgf.nu/";

#[tokio::main]
async fn main() {
    logger::info("Starting up");
    let proxies_raw_txt = fs::read_to_string("proxies.txt").expect("Failed to read proxies.txt");
    let proxies: Vec<&str> = proxies_raw_txt.split("\n").collect();
    if proxies_raw_txt.is_empty() {
        logger::warn("No proxies found in proxies.txt");
    }
    let mut rng = thread_rng();
    let mut i: usize = 0;
    loop {
        i += 1;
        let client = {
            if proxies_raw_txt.is_empty() {
                reqwest::Client::new()
            } else {
                let proxy = proxies[rng.gen_range(0..proxies.len())];
                let proxy = format!("http://{}", proxy);
                reqwest::Client::builder()
                    .proxy(reqwest::Proxy::http(&proxy).unwrap())
                    .build()
                    .unwrap()
            }
        };
        let chars: String = (0..5).map(|_| rng.sample(Alphanumeric) as char).collect();
        let url = format!("{}{}", URL, chars);
        let res = client.get(&url)
            .header("User-Agent", ua_generator::ua::spoof_ua())
            .send()
            .await;
        if let Err(e) = res {
            logger::fatal(&format!("Failed to get URL: {}", e));
            break;
        }
        let res = res.unwrap();
        if res.status() == StatusCode::NOT_FOUND {
            //logger::error(&format!("{}", url));
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        } else if res.status() == StatusCode::OK {
            logger::success(&format!("{}", url));
            logger::info(&format!("{} requests sent", i));
            //break;
        } else {
            logger::fatal(&format!("Unexpected status code: {} {}", res.status(), url));
            let t = res.text().await.unwrap();
            logger::fatal(&format!("Response: {}", t));
            break;
        }
    }
}

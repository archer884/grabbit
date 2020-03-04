mod model;
mod options;
mod reddit;

use reqwest::{self, header, Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options::Opt { user } = options::read();
    let client = build_client()?;

    pages(&client, user).visit_pages(|item| {
        if let Some(url) = item.url() {
            println!("{}", url);
        }
    })?;

    Ok(())
}

fn pages(client: &Client, user: impl Into<String>) -> reddit::UserPages {
    reddit::UserPages::new(client, user)
}

fn build_client() -> reqwest::Result<Client> {
    static USER_AGENT: &str =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:73.0) Gecko/20100101 Firefox/73.0";

    Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(build_default_headers())
        .build()
}

fn build_default_headers() -> header::HeaderMap {
    static ACCEPT_HEADER: &str =
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static(ACCEPT_HEADER),
    );
    headers
}

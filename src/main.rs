use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(
        "postgres://user:pass@localhost:5432/blackhole",
    ));
}

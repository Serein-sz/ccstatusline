use ccstatusline::core::statusline::StatusLine;
use ccstatusline::model::InputData;
use std::io::{self};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let config: InputData = serde_json::from_reader(stdin.lock())?;

    println!("{}", StatusLine::generate(&config).await);
    Ok(())
}

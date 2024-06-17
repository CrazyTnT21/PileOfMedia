use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  navigation::main().await
}

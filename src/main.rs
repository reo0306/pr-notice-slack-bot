use anyhow::Result;

use pullrequest_slack_bot::driver::batch::Batch;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let mut batch = Batch::new();

    batch.run().await.expect("slack bot error");

    Ok(())
}

use fantoccini::{error::CmdError, Client};

use tokio;

async fn to_exec(cl: &mut Client) -> Result<(), CmdError> {
    // web scraping code goes here
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CmdError> {
    example::init_and_exec(to_exec).await
}

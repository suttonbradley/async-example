use fantoccini::Client;
use fantoccini::{error::CmdError, ClientBuilder};

pub async fn init_and_exec<Fut, F>(mut to_exec: F) -> Result<(), CmdError>
where
    Fut: std::future::Future<Output = Result<(), CmdError>>,
    F: FnMut(&mut Client) -> Fut,
{
    // Initialize webdriver client, start webdriver process, execute to_exec, and handle cleanup
    Ok(())
}

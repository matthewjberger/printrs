use anyhow::Result;
use printrs::Printer;

const PLACEHOLDER_TEXT: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

#[async_std::main]
async fn main() -> Result<()> {
    let mut printer = Printer::new("192.168.0.172".to_string(), 9100);

    printer.connect().await?;
    printer.send_raw_bytes(PLACEHOLDER_TEXT.as_bytes()).await?;
    printer.disconnect().await?;

    Ok(())
}

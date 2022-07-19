use anyhow::Result;
use printrs::Printer;

const PLACEHOLDER_TEXT: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit ...
";

#[async_std::main]
async fn main() -> Result<()> {
    let mut printer = Printer::new("192.168.0.172".to_string(), 9100);

    printer.connect().await?;
    printer.send_raw_bytes(PLACEHOLDER_TEXT.as_bytes()).await?;
    printer.disconnect().await?;

    Ok(())
}

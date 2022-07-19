use anyhow::Result;
use printrs::Printer;

const PLACEHOLDER_TEXT_BYTES: &[u8] = b"
Lorem ipsum dolor sit amet, consectetur adipiscing elit ...
";

#[async_std::main]
async fn main() -> Result<()> {
    let mut printer = Printer::new("192.168.0.172".to_string(), 9100);

    printer.connect().await?;
    printer.send_raw_bytes(PLACEHOLDER_TEXT_BYTES).await?;
    printer.disconnect().await?;

    Ok(())
}

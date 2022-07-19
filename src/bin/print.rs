use anyhow::Result;
use printrs::{Printer, Status};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut printer = Printer::new("192.168.0.172".to_string(), 9100);

    printer.connect().await?;

    printer.initialize().await?;

    printer
        .print_text("LLorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...Lorem ipsum dolor sit amet, consectetur adipiscing elit ...orem ipsum dolor sit amet, consectetur adipiscing elit ...")
        .await?;

    printer.cut_paper().await?;

    printer.query_status(Status::Online).await?;

    printer.disconnect().await?;

    Ok(())
}

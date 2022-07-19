use std::net::Shutdown;

use async_std::{net::TcpStream, prelude::*};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("Failed to write bytes to printer.")]
    WriteError(#[from] async_std::io::Error),
}

pub type Result<T, E = PrinterError> = std::result::Result<T, E>;

#[derive(Default, Debug)]
pub struct Printer {
    ip: String,
    port: u16,
    stream: Option<TcpStream>,
}

impl Printer {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            stream: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let stream = TcpStream::connect(format!("{}:{}", self.ip, self.port)).await?;
        stream.set_nodelay(true)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn print_text(&mut self, text: &str) -> Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            stream.write_all(text.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            stream.shutdown(Shutdown::Both)?;
        }
        self.stream = None;
        Ok(())
    }
}

use std::net::Shutdown;

use async_std::{net::TcpStream, prelude::*};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("Failed to write bytes to printer.")]
    WriteError(#[from] async_std::io::Error),

    #[error("Failed to read/write to the printer. The printer is disconnected.")]
    NotConnected,
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
        if self.stream.is_some() {
            self.disconnect().await?;
        }

        let stream = TcpStream::connect(format!("{}:{}", self.ip, self.port)).await?;
        stream.set_nodelay(true)?;
        self.stream = Some(stream);

        Ok(())
    }

    pub async fn print_text(&mut self, text: &str) -> Result<()> {
        self.stream_mut()?.write_all(text.as_bytes()).await?;
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.stream_mut()?.shutdown(Shutdown::Both)?;
        self.stream = None;
        Ok(())
    }

    fn stream_mut(&mut self) -> Result<&mut TcpStream> {
        self.stream.as_mut().ok_or(PrinterError::NotConnected)
    }
}

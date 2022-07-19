use async_std::{net::TcpStream, prelude::*};
use std::net::Shutdown;
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

    pub async fn disconnect(&mut self) -> Result<()> {
        self.stream_mut()?.shutdown(Shutdown::Both)?;
        self.stream = None;
        Ok(())
    }

    pub async fn cut_paper(&mut self) -> Result<()> {
        self.execute_command(Command::CutPaper).await?;
        Ok(())
    }

    pub async fn print_text(&mut self, text: &str) -> Result<()> {
        let command = Command::PrintText(text.to_string());
        self.execute_command(command).await
    }

    pub async fn initialize(&mut self) -> Result<()> {
        self.execute_command(Command::Initialize).await
    }

    pub async fn select(&mut self) -> Result<()> {
        self.execute_command(Command::Select).await
    }

    pub async fn query_status(&mut self, status: Status) -> Result<()> {
        self.execute_command_with_response(Command::QueryStatus(status))
            .await
    }

    fn stream_mut(&mut self) -> Result<&mut TcpStream> {
        self.stream.as_mut().ok_or(PrinterError::NotConnected)
    }

    async fn execute_command(&mut self, command: Command) -> Result<()> {
        log::info!("executing command: {}", command.raw());
        self.send_raw_bytes(command.raw().as_bytes()).await?;
        Ok(())
    }

    async fn execute_command_with_response(&mut self, command: Command) -> Result<()> {
        self.execute_command(command).await?;
        // task::sleep(Duration::from_secs(1)).await;
        // let mut buffer = vec![0u8; 1024];
        // self.stream_mut()?.read(&mut buffer).await?;
        // log::info!("response {:?}", buffer);
        Ok(())
    }

    async fn send_raw_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.stream_mut()?.write_all(bytes).await?;
        Ok(())
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        if let Some(stream) = self.stream.as_mut() {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

pub enum ControlSequence {
    LineFeed,
    FormFeed,
    CarriageReturn,
    HorizontalTab,
    VerticalTab,
}

impl ControlSequence {
    pub fn code(&self) -> String {
        match self {
            ControlSequence::LineFeed => "0A",
            ControlSequence::FormFeed => "0C",
            ControlSequence::CarriageReturn => "0D",
            ControlSequence::HorizontalTab => "09",
            ControlSequence::VerticalTab => "0B",
        }
        .to_string()
    }
}

pub enum ControlCharacters {
    CAN,
    DC4,
    DLE,
    ENQ,
    EOT,
    ESC,
    FS,
    GS,
    NUL,
}

impl ControlCharacters {
    #[rustfmt::skip]
    pub fn code(&self) -> String {
        match self {
            ControlCharacters::CAN => "\x18",
            ControlCharacters::DC4 => "\x14",
            ControlCharacters::DLE => "\x10",
            ControlCharacters::ENQ => "\x05",
            ControlCharacters::EOT => "\x04",
            ControlCharacters::ESC => "\x1b",
            ControlCharacters::FS  => "\x1c",
            ControlCharacters::GS  => "\x1d",
            ControlCharacters::NUL => "\x00",
        }.to_string()
    }
}

#[derive(Debug)]
pub enum Command {
    CutPaper,
    Initialize,
    PrintText(String),
    QueryStatus(Status),
    Select,
}

impl Command {
    pub fn raw(&self) -> String {
        let escape_code = ControlCharacters::ESC.code();
        match self {
            Command::CutPaper => format!("{}V1", ControlCharacters::GS.code()),
            Command::Initialize => format!("{}@", escape_code),
            Command::PrintText(text) => format!("{}{}", ControlSequence::LineFeed.code(), text),
            Command::Select => format!("{}\x01", escape_code),
            Command::QueryStatus(status) => status.command(),
        }
        .to_string()
    }
}

#[derive(Debug)]
pub enum Status {
    Online,
    Paper,
}

impl Status {
    pub fn command(&self) -> String {
        let status_prefix = format!(
            "{}{}",
            ControlCharacters::EOT.code(),
            ControlCharacters::DLE.code()
        );
        match self {
            Status::Online => format!("{}\x01", status_prefix),
            Status::Paper => format!("{}\x04", status_prefix),
        }
        .to_string()
    }
}

pub enum StatusMask {
    Online,
    Paper,
    LowPaper,
    NoPaper,
}

impl StatusMask {
    pub fn mask(&self) -> usize {
        match self {
            StatusMask::Online => 8,
            StatusMask::Paper => 18,
            StatusMask::LowPaper => 30,
            StatusMask::NoPaper => 114,
        }
    }
}

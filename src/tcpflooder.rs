use std::net::TcpStream;
use std::io::Write;

pub struct TcpFlooder {
    target: String,
    port: u32

}

impl TcpFlooder {
    pub fn flood(&self) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.target, self.port))?;
        stream.write(b"asd")?;
        Ok(())
    }
    pub fn new(target: String, port: u32) -> Self {
        TcpFlooder {
            target,
            port
        }
    }
}
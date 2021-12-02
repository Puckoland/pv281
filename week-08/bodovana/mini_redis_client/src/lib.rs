use command::Command;
use tokio::{
    io::{self, AsyncWriteExt, Interest},
    net::TcpStream,
};

pub async fn write_command_to_stream(stream: &mut TcpStream, command: Command) -> io::Result<()> {
    let serialized = serde_json::to_string(&command).unwrap();
    stream.write(serialized.as_bytes()).await?;

    Ok(())
}

pub async fn read_from_stream(stream: &mut TcpStream) -> io::Result<()> {
    loop {
        let ready = stream.ready(Interest::READABLE).await?;

        if ready.is_readable() {
            let mut buffer = vec![0; 1024];
            match stream.try_read(&mut buffer) {
                Ok(data_length) => {
                    let data = String::from_utf8(buffer[0..data_length].to_vec()).unwrap();
                    println!("[client] Message from server (dictionary): {}", data);
                    return Ok(());
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

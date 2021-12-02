use command::{Command, Receipt};
use mini_redis_client::{read_from_stream, write_command_to_stream};
use tokio::{
    io,
    net::{TcpSocket, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let receipt1 = Receipt::new(150, Some("Dinner"));
    let receipt2 = Receipt::new(10, None);
    let receipt3 = Receipt::new(200, Some("Lunch"));
    let commands = vec![
        Command::create_get_command_from(111),
        Command::create_insert_command_from(111, receipt1),
        Command::create_get_command_from(111),
        Command::create_insert_command_from(222, receipt2),
        Command::create_insert_command_from(333, receipt3),
        Command::create_get_command_from(333),
        Command::create_get_command_from(222),
    ];

    let mut stream = connect_to_server().await?;

    // First command attempts to get a value which is not in the dictionary.
    // The second one inserts the value for given key, third command just to be
    // sure it is already there :)
    for command in commands {
        write_command_to_stream(&mut stream, command).await?;
        read_from_stream(&mut stream).await?;
    }

    Ok(())
}

async fn connect_to_server() -> io::Result<TcpStream> {
    let addr = "127.0.0.1:8000".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;

    Ok(stream)
}

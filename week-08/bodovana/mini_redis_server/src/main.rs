use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{self, AsyncReadExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

mod dictionary;
use dictionary::{get_from_dict, insert_to_dict};

use command::{Command, CommandType, Receipt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let dictionary: Arc<RwLock<HashMap<usize, Receipt>>> = Arc::new(RwLock::new(HashMap::new()));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let dict_clone = dictionary.clone();
        process_socket(&mut socket, dict_clone).await?;
    }
}

async fn process_socket(
    socket: &mut TcpStream,
    dictionary: Arc<RwLock<HashMap<usize, Receipt>>>,
) -> io::Result<()> {
    let mut buffer = vec![0; 1024];

    loop {
        let data_result = socket.read(&mut buffer).await;

        let data_length = match data_result {
            // There is nothing more to read
            Ok(0) => {
                break;
            }
            Ok(size) => size,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(err) => {
                return Err(err);
            }
        };

        // We should use match instead of unwrap
        let data = String::from_utf8(buffer[0..data_length].to_vec()).unwrap();
        process_command(socket, data, dictionary.clone()).await?;
    }

    Ok(())
}

async fn process_command(
    socket: &mut TcpStream,
    data_string: String,
    dictionary: Arc<RwLock<HashMap<usize, Receipt>>>,
) -> io::Result<()> {
    let deserialized: Command = serde_json::from_str(&data_string).unwrap();
    println!("[server] Command received from client: {:?}", deserialized);

    match deserialized.cmd {
        CommandType::Get => get_from_dict(socket, dictionary, deserialized.key).await?,
        CommandType::Insert => {
            insert_to_dict(
                socket,
                dictionary,
                deserialized.key,
                deserialized.value.unwrap(),
            )
            .await?
        }
    };

    Ok(())
}

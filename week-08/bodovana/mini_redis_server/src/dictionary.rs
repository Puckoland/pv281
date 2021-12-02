use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream,
    sync::RwLock,
};

use command::Receipt;

pub(crate) async fn get_from_dict(
    socket: &mut TcpStream,
    dictionary: Arc<RwLock<HashMap<usize, Receipt>>>,
    key: usize,
) -> io::Result<()> {
    // unlock for reading
    let read_dictionary = dictionary.read().await;
    let item: Option<&Receipt> = read_dictionary.get(&key);

    let serialized;
    let to_send = match item {
        Some(v) => {
            serialized = serde_json::to_string(v).unwrap().clone();
            serialized.as_bytes()
        },
        None => b"Item not found",
    };

    socket.write(to_send).await?;
    Ok(())
}

pub(crate) async fn insert_to_dict(
    socket: &mut TcpStream,
    dictionary: Arc<RwLock<HashMap<usize, Receipt>>>,
    key: usize,
    value: Receipt,
) -> io::Result<()> {
    // unlock for writing
    let mut write_dicitonary = dictionary.write().await;
    let to_send = format!("Pair {} {:?} inserted", key, &value);
    write_dicitonary.insert(key, value);

    socket.write(to_send.as_bytes()).await?;
    Ok(())
}

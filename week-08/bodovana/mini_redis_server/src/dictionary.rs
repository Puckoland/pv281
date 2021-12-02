use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream,
};

use command::Receipt;

pub(crate) async fn get_from_dict(
    socket: &mut TcpStream,
    key: usize,
) -> io::Result<()> {
    let item: Option<Receipt> = Receipt::load(&key);

    let serialized;
    let to_send = match item {
        Some(v) => {
            serialized = serde_json::to_string(&v).unwrap().clone();
            serialized.as_bytes()
        },
        None => b"Item not found",
    };

    socket.write(to_send).await?;
    Ok(())
}

pub(crate) async fn insert_to_dict(
    socket: &mut TcpStream,
    key: usize,
    value: Receipt,
) -> io::Result<()> {
    let to_send = format!("Pair {} {:?} inserted", key, &value);
    value.save(&key);

    socket.write(to_send.as_bytes()).await?;
    Ok(())
}

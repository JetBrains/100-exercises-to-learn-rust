// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
// `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let reply = Arc::new(reply);
    let handle1 = tokio::spawn(_fixed_reply(first, Arc::clone(&reply)));
    let handle2 = tokio::spawn(_fixed_reply(second, reply));

    tokio::join!(handle1, handle2,);
}

async fn _fixed_reply<T>(listener: TcpListener, reply: Arc<T>)
where
    T: Display + Send + Sync + 'static,
{
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let (_reader, mut writer) = socket.split();
        writer
            .write_all(format!("{}", reply).as_bytes())
            .await
            .unwrap();
    }
}
use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    let handle1 = tokio::spawn(echo(first));
    let handle2 = tokio::spawn(echo(second));
    let (outcome1, outcome2) = tokio::join!(handle1, handle2);
    outcome1??;
    outcome2??;
    Ok(())
}

async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}

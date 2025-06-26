// TODO: the `echo` server uses non-async primitives.
//  When running the tests, you should observe that it hangs, due to a
//  deadlock between the caller and the server.
//  Use `spawn_blocking` inside `echo` to resolve the issue.
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::task::spawn_blocking;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket = socket.into_std()?;
        spawn_blocking(move || -> Result<(), anyhow::Error> {
            socket.set_nonblocking(false)?;
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer)?;
            socket.write_all(&buffer)?;
            Ok(())
        });
    }
}

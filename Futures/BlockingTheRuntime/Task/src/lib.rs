// TODO: сервер `echo` використовує неасинхронні примітиви.
//  Запустивши тести, ви побачите, що він зависає через
//  deadlock між тим, хто викликає, і сервером.
//  Використайте `spawn_blocking` всередині `echo`, щоб розв'язати проблему.
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

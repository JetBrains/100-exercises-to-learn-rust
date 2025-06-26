// TODO: fix the `assert_eq` at the end of the tests.
//  Do you understand why that's the resulting output?

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use task_cancelation::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let messages = vec!["hello", "from", "this", "task"];
        let timeout = Duration::from_millis(20);
        let handle = tokio::spawn(run(listener, messages.len(), timeout.clone()));

        for message in messages {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (_, mut writer) = socket.split();

            let (beginning, end) = message.split_at(message.len() / 2);

            // Send first half
            writer.write_all(beginning.as_bytes()).await.unwrap();
            tokio::time::sleep(timeout * 2).await;
            writer.write_all(end.as_bytes()).await.unwrap();

            // Close the write side of the socket
            let _ = writer.shutdown().await;
        }

        let buffered = handle.await.unwrap();
        let buffered = std::str::from_utf8(&buffered).unwrap();
        assert_eq!(buffered, "hefrthta");
    }
}

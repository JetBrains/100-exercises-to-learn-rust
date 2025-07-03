#[cfg(test)]
mod tests {
    use task_async_aware_primitives::{pong, Message};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(1);
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        
        sender
            .send(Message::new("pong".into(), response_sender))
            .await
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload();
        assert_eq!(answer, "pong");
    }
}
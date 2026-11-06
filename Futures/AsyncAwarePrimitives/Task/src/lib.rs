/// TODO: код нижче потрапить у взаємне блокування (deadlock), бо використовує канали з std,
///  які не підтримують async.
///  Перепишіть його з використанням примітиву каналів `tokio` (так, доведеться
///  змінити й тестовий код).
///
/// Чи зможете ви зрозуміти, яка послідовність подій може призвести до взаємного блокування?
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

impl Message {
    pub fn new(payload: String, response_channel: mpsc::Sender<Message>) -> Self {
        Message {
            payload: payload.into(),
            response_channel,
        }
    }

    pub fn payload(self) -> String {
        self.payload
    }
}

/// Відповідає `pong` на будь-яке отримане повідомлення, створюючи новий
/// канал, щоб і далі спілкуватися з тим, хто викликає.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(1);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}

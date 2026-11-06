// TODO: реалізуйте функцію `fixed_reply`. Вона має приймати два екземпляри `TcpListener`,
//  конкурентно приймати з'єднання на обох і завжди відповідати клієнтам, надсилаючи
//  у відповідь представлення аргументу `reply`, отримане через `Display`.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` не можна клонувати. Як спільно використовувати його у двох серверних tasks?
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

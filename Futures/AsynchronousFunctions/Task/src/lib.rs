use tokio::net::TcpListener;

// TODO: напишіть ехо-сервер, який приймає вхідні TCP-з'єднання та
//  надсилає клієнту отримані дані назад.
//  `echo` не повинна повертати керування після завершення обробки з'єднання, а має
//  й далі приймати нові з'єднання.
//
// Підказка: для реалізації ехо-сервера використовуйте структури й методи `tokio`.
// Зокрема:
// - `tokio::net::TcpListener::accept` — щоб обробити наступне вхідне з'єднання;
// - `tokio::net::TcpStream::split` — щоб отримати із сокета зчитувач (reader) і записувач (writer);
// - `tokio::io::copy` — щоб скопіювати дані зі зчитувача в записувач.
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}

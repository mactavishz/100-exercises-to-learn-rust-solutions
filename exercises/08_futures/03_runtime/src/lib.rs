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
    let arc_reply = Arc::new(reply);

    let first_reply = Arc::clone(&arc_reply);
    let first_handle = tokio::spawn(async move {
        loop {
            if let Ok((mut socket, _)) = first.accept().await {
                let reply = Arc::clone(&first_reply);
                tokio::spawn(async move {
                    let (_, mut writer) = socket.split();
                    let message = reply.as_ref().to_string();
                    writer
                        .write_all(message.as_bytes())
                        .await
                        .expect("Failed to write reply");
                });
            }
        }
    });

    let second_reply = Arc::clone(&arc_reply);
    let second_handle = tokio::spawn(async move {
        loop {
            if let Ok((mut socket, _)) = second.accept().await {
                let reply = Arc::clone(&second_reply);
                tokio::spawn(async move {
                    let (_, mut writer) = socket.split();
                    let message = reply.as_ref().to_string();
                    writer
                        .write_all(message.as_bytes())
                        .await
                        .expect("Failed to write reply");
                });
            }
        }
    });

    let _ = tokio::try_join!(first_handle, second_handle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}

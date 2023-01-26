use crate::shared::protocol::Message;
use crate::shared::solution::Solution;
use crate::shared::ADDR;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = TcpStream::connect(&ADDR).await?;
    let transport = Framed::new(socket, LengthDelimitedCodec::new());

    let (mut w, mut r) = transport.split();

    // Send Hello.
    let message = Message::Hello.try_into()?;
    w.send(message).await?;

    // Receive challenge.
    let challenge = loop {
        match r.next().await {
            Some(Ok(message)) => {
                let message: Message = message.try_into()?;
                if let Message::Challenge(challenge) = message {
                    break challenge;
                }
            }
            Some(Err(..)) => anyhow::bail!("Error polling the connection"),
            None => anyhow::bail!("Connection closed, challenge has not been received"),
        }
    };
    println!("Challenge received: `{challenge:?}`");

    // Solve Challenge.
    let solution = Solution::find(&challenge)?;

    // Submit the solution.
    let message = Message::Solution(solution).try_into()?;
    w.send(message).await?;

    // Receive wisdom.
    let wisdom = loop {
        match r.next().await {
            Some(Ok(message)) => {
                let message: Message = message.try_into()?;
                if let Message::Wisdom(wisdom) = message {
                    break wisdom;
                }
            }
            Some(Err(..)) => anyhow::bail!("Error polling the connection"),
            None => anyhow::bail!("Connection closed, wisdom has not been received"),
        }
    };

    println!("Wisdom received: `{wisdom}`");

    Ok(())
}

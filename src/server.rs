mod shared;

use crate::shared::protocol::Message;
use crate::shared::solution::{Challenge, VerificationStatus};
use crate::shared::ADDR;
use futures::{SinkExt, StreamExt};
use rand::Rng;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const BOOK: [&str; 2] = [
    "Observe sunset at least once a day",
    "You only get one chance to make a first impression",
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(ADDR).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(err) = handle(socket).await {
                println!("Error when handling connection: {err:?}");
            }
        });
    }
}

async fn handle(socket: TcpStream) -> anyhow::Result<()> {
    let transport = Framed::new(socket, LengthDelimitedCodec::new());

    let (mut w, mut r) = transport.split();

    // Expect Hello.
    loop {
        match r.next().await {
            Some(Ok(message)) => {
                let message: Message = message.try_into()?;
                if matches!(message, Message::Hello) {
                    break;
                }
            }
            Some(Err(..)) => anyhow::bail!("Error polling the connection"),
            None => anyhow::bail!("Connection closed, challenge has not been received"),
        }
    }

    // Send challenge.
    let challenge = Challenge::random(1);
    let message = Message::Challenge(challenge.clone()).try_into()?;
    w.send(message).await?;

    // Expect solution.
    let solution = loop {
        match r.next().await {
            Some(Ok(message)) => {
                let message: Message = message.try_into()?;
                if let Message::Solution(solution) = message {
                    break solution;
                }
            }
            Some(Err(..)) => anyhow::bail!("Error polling the connection"),
            None => anyhow::bail!("Connection closed, challenge has not been received"),
        }
    };

    // Verify solution.
    match solution.verify(&challenge)? {
        VerificationStatus::Valid(hash) => println!(
            "Solution addresses the challenge:\n{challenge:?}\n{solution:?}\nhash: {hash:?}"
        ),
        VerificationStatus::Invalid => anyhow::bail!("Solution DOES NOT address the challenge"),
    }

    // Send Wisdom.
    let wisdom = {
        let mut rng = rand::thread_rng();
        BOOK[rng.gen_range(0..BOOK.len())].to_string()
    };
    let message = Message::Wisdom(wisdom).try_into()?;
    w.send(message).await?;

    Ok(())
}

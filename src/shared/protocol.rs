use crate::shared::solution::{Challenge, Solution};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Challenge(Challenge),
    Solution(Solution),
    Wisdom(String),
}

impl TryFrom<&Message> for Bytes {
    type Error = bincode::Error;

    fn try_from(value: &Message) -> Result<Self, Self::Error> {
        bincode::serialize(value).map(|b| b.into())
    }
}

impl TryFrom<Message> for Bytes {
    type Error = bincode::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        bincode::serialize(&value).map(|b| b.into())
    }
}

impl TryFrom<BytesMut> for Message {
    type Error = bincode::Error;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        bincode::deserialize::<Message>(&value)
    }
}

use std::io;

use bytes::{BufMut, Bytes, BytesMut};
use tokio_io::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Identity;

impl Encoder for Identity {
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.len());
        dst.put(item);
        Ok(())
    }
}

impl Decoder for Identity {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(src.take().freeze()))
    }
}

use bincode;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    io::{Read, Write},
    net::TcpListener,
};
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize, Serialize)]
struct Message(Vec<u8>, Vec<u8>);

static KEY: &'static str = "secret key";
impl Message {
    fn new(data: &str, key: &[u8]) -> Self {
        let data = data.as_bytes();
        let hmac = create_hmac(data, key);
        Message(data.to_vec(), hmac)
    }
    fn verify(&self) -> bool {
        let mut mac = HmacSha256::new_from_slice(KEY.as_bytes()).unwrap();
        mac.update(&self.0);
        mac.verify((&self.1[..]).into()).is_ok()
    }
}
fn create_hmac(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(KEY.as_bytes()).unwrap();
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}
fn main() {
    let message = "hello my name is mralians";
    let hmac = Message::new(message, KEY.as_bytes());
    let bin = bincode::serialize(&hmac).unwrap();
    let mut socket = std::net::TcpStream::connect("0.0.0.0:8081").unwrap();
    socket.write_all(&bin).unwrap();
    drop(socket);
}

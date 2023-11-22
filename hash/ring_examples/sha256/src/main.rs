use ring::digest;
use std::{fs::File, io::Read};
fn main() {
    let mut buf = String::new();
    File::open("/etc/passwd")
        .and_then(|mut f| f.read_to_string(&mut buf))
        .unwrap();
    let h = digest::digest(&digest::SHA256, buf.as_bytes());
    println!("{h:?}");
}

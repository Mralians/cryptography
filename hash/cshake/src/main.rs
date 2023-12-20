use base64::encode;
use sha3::{
    digest::{ExtendableOutput, Update},
    Shake128,
};
use std::io::Read;
fn main() {
    let mut hasher = Shake128::default();
    hasher.update(b"ali shojaee the nwms boss");
    let mut reader = hasher.finalize_xof();
    let mut res1 = [0u8; 40];
    let _ = reader.read(&mut res1);
    let rand = encode(res1);
    println!("{rand}");
}

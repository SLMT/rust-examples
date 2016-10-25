extern crate poa;

use poa::data::{Block, Message};
use poa::cipher::Decrypter;

fn main() {
    let key = Block::parse("72, 113, 247, 237, 50, 128, 129, 50, 92, 211, 67, 70, 24, 51, 94, 185");

    let mut encrypted_msg: Message = Vec::new();
    encrypted_msg.push(Block::parse("188, 29, 32, 14, 191, 52, 30, 4, 198, 112, 121, 95, 156, 202, 62, 48"));
    encrypted_msg.push(Block::parse("189, 75, 182, 142, 173, 213, 191, 85, 251, 215, 20, 28, 129, 252, 101, 140"));

    let decrypter = Decrypter::new(key.clone());
    let result = decrypter.decrypt(&encrypted_msg);
    println!("Result: {:?}", result);
}

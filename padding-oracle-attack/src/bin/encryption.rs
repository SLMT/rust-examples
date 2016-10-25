extern crate poa;

use poa::data::Block;
use poa::cipher::Encrypter;

fn main() {
    let iv = Block::parse("188, 29, 32, 14, 191, 52, 30, 4, 198, 112, 121, 95, 156, 202, 62, 48");
    let key = Block::parse("72, 113, 247, 237, 50, 128, 129, 50, 92, 211, 67, 70, 24, 51, 94, 185");
    let message = "I'am a cat.";

    let encrypter = Encrypter::new(key.clone(), iv.clone());
    println!("IV: {:?}", iv);
    println!("Key: {:?}", key);

    let encrypted_msg = encrypter.encrypt(message);
    println!("Cipher: {:?}", encrypted_msg);
}

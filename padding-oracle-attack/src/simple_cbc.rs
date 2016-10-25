
use data::Block;
use data::Message;

pub const BLOCK_SIZE: usize = 16;

pub fn generate_iv() -> Block {
    Block::random_generate(BLOCK_SIZE)
}

pub fn generate_key() -> Block {
    Block::random_generate(BLOCK_SIZE)
}

pub fn encrypt(iv: &Block, key: &Block, plaintext: &Message) -> Message {
    let mut ciphertext: Message = Vec::new();
    ciphertext.push(iv.clone());

    for blk_idx in 0..plaintext.len() {
        let intermediate_blk = &ciphertext[blk_idx] ^ &plaintext[blk_idx];
        let encrypted_blk = &intermediate_blk ^ key;
        ciphertext.push(encrypted_blk);
    }

    ciphertext
}

pub fn decrypt(key: &Block, ciphertext: &Message) -> Message {
    let mut plaintext: Message = Vec::new();

    for blk_idx in 1..ciphertext.len() {
        let intermediate_blk = &ciphertext[blk_idx] ^ key;
        let decrypted_blk = &intermediate_blk ^ &ciphertext[blk_idx - 1];
        plaintext.push(decrypted_blk);
    }

    plaintext
}


use pkcs7;
use simple_cbc;
use data::Block;
use data::Message;

pub struct Encrypter {
    key: Block,
    iv: Block
}

impl Encrypter {
    pub fn new(key: Block, iv: Block) -> Encrypter {
		Encrypter {
            key: key,
            iv: iv
        }
    }

    pub fn encrypt(&self, msg: &str) -> Message {
        let padded_msg = pkcs7::add_padding(msg, simple_cbc::BLOCK_SIZE);
        simple_cbc::encrypt(&self.iv, &self.key, &padded_msg)
    }
}

pub struct Decrypter {
    key: Block
}

impl Decrypter {
    pub fn new(key: Block) -> Decrypter {
		Decrypter {
            key: key
        }
    }

    pub fn decrypt(&self, msg: &Message) -> String {
        let decrypted = simple_cbc::decrypt(&self.key, msg);
        pkcs7::remove_padding(&decrypted)
    }
}

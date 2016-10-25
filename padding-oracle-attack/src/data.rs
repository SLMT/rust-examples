
use std::ops::BitXor;
use std::str::FromStr;

use rand;
use rand::Rng;

pub type Message = Vec<Block>;

#[derive(Debug)]
pub struct Block {
    bytes: Vec<u8>
}

impl Block {
    pub fn from_array(arr: &[u8]) -> Block {
        Block {
            bytes: Vec::from(arr)
        }
    }

    pub fn random_generate(length: usize) -> Block {
        let mut random = rand::thread_rng();
        let data = random.gen_iter::<u8>().take(length).collect::<Vec<u8>>();

        Block {
            bytes: data
        }
    }

    // XXX: It should be implemented in the trait "FromStr",
    // but I am too lazy to implement error messages
    pub fn parse(string: &str) -> Block {
        let bytes: Vec<u8> = string.split(',').map(|s| u8::from_str(s.trim()).unwrap()).collect();

        Block {
            bytes: bytes
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl<'a, 'b> BitXor<&'a Block> for &'b Block {
    type Output = Block;

    fn bitxor(self, _rhs: &Block) -> Block {
        let len = self.bytes.len();
        let mut out: Vec<u8> = Vec::new();

        for idx in 0..len {
            out.push(self.bytes[idx] ^ _rhs.bytes[idx]);
        }

        Block {
            bytes: out
        }
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        Block {
            bytes: self.bytes.clone()
        }
    }
}

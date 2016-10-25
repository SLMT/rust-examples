
use data::Block;
use data::Message;

pub fn add_padding(s: &str, block_size: usize) -> Message {
    let mut message: Message = Vec::new();
    let bytes = s.bytes().collect::<Vec<u8>>();

    // Copy the bytes of string
    let mut front: &[u8] = &[];
    let mut back = bytes.as_slice();
    while back.len() >= block_size {
        message.push(Block::from_array(front));
        let (a, b) = back.split_at(block_size);
        front = a;
        back = b;
    }

    // Add padding
    let padding: u8 = (block_size - back.len()) as u8;
    let mut last_block: Vec<u8> = Vec::from(back);
    for _ in 0..padding {
        last_block.push(padding);
    }
    message.push(Block::from_array(last_block.as_slice()));

    message
}

pub fn remove_padding(m: &Message) -> String {
    let block_size = m[0].len();

    // Transform to a vector
    let mut bytes: Vec<u8> = Vec::new();
    for blk in m {
        bytes.extend_from_slice(blk.as_slice());
    }

    // Remove the padding
    let padding = bytes.last().unwrap().clone();
    let (main, _) = bytes.split_at(block_size - (padding as usize));

    String::from_utf8(Vec::from(main)).unwrap()
}

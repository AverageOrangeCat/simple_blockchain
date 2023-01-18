mod block;
use block::Block;

mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    let mut block_01 = Block {
        previous_hash: String::new(),
        current_hash: String::new(),
        content: String::from("Hello, World!"),
        nounce: 0
    };

    let mut block_02 = Block {
        previous_hash: String::new(),
        current_hash: String::new(),
        content: String::from("Hello, World!"),
        nounce: 0
    };

    let mut block_03 = Block {
        previous_hash: String::new(),
        current_hash: String::new(),
        content: String::from("Hello, World!"),
        nounce: 0
    };

    block_01.previous_hash = String::new();
    block_01.mine_block();
    block_01.debug();

    block_02.previous_hash = block_01.current_hash.clone();
    block_02.mine_block();
    block_02.debug();

    block_03.previous_hash = block_02.current_hash.clone();
    block_03.mine_block();
    block_03.debug();

    blockchain.blocks.push(block_01);
    blockchain.blocks.push(block_02);
    blockchain.blocks.push(block_03);
    blockchain.debug();
}

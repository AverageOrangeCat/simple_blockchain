mod block;
use block::Block;

fn main() {
    let mut block = Block {
        previous_hash: String::new(),
        current_hash: String::new(),
        content: String::from("Hello, World!"),
        nounce: 0
    };

    block.mine_block();
    block.debug(String::new());
}

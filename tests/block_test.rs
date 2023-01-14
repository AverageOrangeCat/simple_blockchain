use simple_blockchain::block::Block;

#[test]
fn test_mine_block() -> () {
    let mined_block = Block {
        previous_hash: String::from(""),
        current_hash: String::from("00007e9a19778e5c0b9d8cd448707b1929e284389f738924d0517ce0cb132adb"),
        content: String::from("Hello, World!"),
        nounce: 29861
    };

    let mut new_block = Block {
        previous_hash: String::new(),
        current_hash: String::new(),
        content: String::from("Hello, World!"),
        nounce: 0
    };

    new_block.mine_block();

    assert_eq!(mined_block.previous_hash, new_block.previous_hash);
    assert_eq!(mined_block.current_hash, new_block.current_hash);
    assert_eq!(mined_block.content, new_block.content);
    assert_eq!(mined_block.nounce, new_block.nounce);
}

#[test]
fn test_validate() -> () {
    let invalid_block = Block {
        previous_hash: String::from(""),
        current_hash: String::from("00007e9d8c9f738924d0517ce0cb132adbd448707b1929e28438a19778e5c0b9"),
        content: String::from("Hello, World!"),
        nounce: 38475
    };

    let valid_block = Block {
        previous_hash: String::from(""),
        current_hash: String::from("00007e9a19778e5c0b9d8cd448707b1929e284389f738924d0517ce0cb132adb"),
        content: String::from("Hello, World!"),
        nounce: 29861
    };

    assert_eq!(invalid_block.validate(String::from("")), false);
    assert_eq!(valid_block.validate(String::from("")), true);
}

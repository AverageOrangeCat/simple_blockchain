use super::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: Vec::new()
        }
    }

    fn validate_references(&self) -> bool {
        for block_slice in self.blocks.windows(2) {
            if block_slice[0].current_hash != block_slice[1].previous_hash {
                return false;
            }
        }

        true
    }

    fn validate_blocks(&self) -> bool {
        for block in &self.blocks {
            if block.validate() != true {
                return false;
            }
        }

        true
    }

    pub fn validate(&self) -> bool {
        self.validate_references() &&
        self.validate_blocks()
    }

    pub fn debug(&self) -> () {
        println!("[ DEBUG ] is_valid: {:?}", self.validate());
        println!("[ DEBUG ] blocks_len: {:?}", self.blocks.len());
        println!("[ DEBUG ] ");
    }
}

use sha3::Digest;
use sha3::Sha3_256;
use hex::encode;

const DIFFICULT_PREFIX: &str = "0000";

#[derive(Debug)]
pub struct Block {
    pub previous_hash: String,
    pub current_hash: String,
    pub content: String,
    pub nounce: u64
}

impl Block {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha3_256::new();

        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.content.as_bytes());
        hasher.update(self.nounce.to_le_bytes());

        let result = hasher.finalize();

        encode(result)
    }

    pub fn mine_block(&mut self) -> () {
        while !self.current_hash.starts_with(DIFFICULT_PREFIX) {
            self.nounce += 1;
            self.current_hash = self.calculate_hash();
        }
    }

    pub fn validate(&self) -> bool {
        self.current_hash == self.calculate_hash() &&
        self.current_hash.starts_with(DIFFICULT_PREFIX)
    }

    pub fn debug(&self) -> () {
        println!("[ DEBUG ] is_valid: {:?}", self.validate());
        println!("[ DEBUG ] previous_hash: {:?}", self.previous_hash);
        println!("[ DEBUG ] current_hash: {:?}", self.current_hash);
        println!("[ DEBUG ] content: {:?}", self.content);
        println!("[ DEBUG ] nounce: {:?}", self.nounce);
        println!("[ DEBUG ] ");
    }
}

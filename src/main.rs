use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0;32], 0, "Genesis block!".to_owned(), difficulty);
    block.mine();

    let mut last_hash = block.hash.clone();

    println!("Mined genesis block {:?}", &block);

    let mut blockchain = Blockchain{
        blocks: vec![block], 
    };
    println!("Verify: {}", &blockchain.verify());

    // =10 means 1-10 *inclusive*
    for i in 1..=10{
        let mut block = Block::new(i, now(), last_hash, 0, "another block".to_owned(), difficulty);
        block.mine();
        println!("Minded another block {:?}", &block);
        last_hash = block.hash();
        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }

    blockchain.blocks[3].index = 5;
    println!("Verify: {}", &blockchain.verify());
}

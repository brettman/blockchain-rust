use blockchainlib::*;

fn main() {
    println!("Hello, blockchain!");
    let mut block = Block::new(12, now(), vec![0;32], 0, "Genesis block!".to_owned());

    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);

    block.hash = h;
    println!("{:?}", &block);
}

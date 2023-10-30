mod block;
mod blockchain;
mod cli;
mod errors;
mod transaction;

// use blockchain::Blockchain;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())

    //     let mut blockchain = Blockchain::new().unwrap();

    //     blockchain.add_block("Data 1".to_string()).unwrap();
    //     blockchain.add_block("Data 2".to_string()).unwrap();
    //     blockchain.add_block("Data 3".to_string()).unwrap();

    //     for block in blockchain.iter() {
    //         println!("{:?}", block);
    //     }
}

use core::panic;

use clap::Parser;
use csv::Reader;
use jatp::transactions::transaction::Transaction;
use jatp::tx_manager::TxManager;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _manager = TxManager::new();

    let mut rdr = Reader::from_path(&cli.file)?;

    for item in rdr.deserialize::<Transaction>() {
        match item {
            Ok(tx) => println!("{:?}", tx),
            Err(e) => println!("oh no an invalid tx {:?}", e),
        }
    }

    Ok(())
}

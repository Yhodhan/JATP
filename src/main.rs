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
    let mut tx_manager = TxManager::new();
    let mut rdr = Reader::from_path(&cli.file)?;

    for tx in rdr.deserialize::<Transaction>() {
        match tx {
            Ok(t) => tx_manager.process_tx(t),
            Err(_) => unimplemented!(),
        }
    }

    for (_id, acc) in tx_manager.accounts {
        println!("acc {:?}", acc);
    }

    Ok(())
}

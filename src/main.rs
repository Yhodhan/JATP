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
            Err(e) => eprintln!("skipping invalid tx: {}", e),
        }
    }

    println!("client, available, held, total, locked");
    for account in tx_manager.accounts.values() {
        println!(
            "{}, {:.4}, {:.4}, {:.4}, {}",
            account.id, account.available, account.held, account.total, account.locked
        );
    }

    Ok(())
}

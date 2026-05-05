struct Transactions {
    tx_type: String, 
    account_id: u32, 
}

impl Transactions {
    pub fn new (tx_type: &str, account_id: u32) -> Self {
        Transactions { tx_type: String::from(tx_type), account_id }
    }
}
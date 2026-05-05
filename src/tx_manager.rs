use std::collections::HashMap;

use crate::accounts::account::Account;
use crate::transactions::transaction::Transaction;
pub struct TxManager {
    accounts: HashMap<u32, Account>,
    transactions: HashMap<u32, Transaction>,
}

impl TxManager {
    pub fn new() -> Self {
        TxManager {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }
}

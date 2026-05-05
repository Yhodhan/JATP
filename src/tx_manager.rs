use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::accounts::account::Account;
use crate::transactions::transaction::Transaction;
use crate::transactions::transaction::TxType;
pub struct TxManager {
    pub accounts: HashMap<u16, Account>,
    pub transactions: HashMap<u32, Transaction>,
}

impl TxManager {
    pub fn new() -> Self {
        TxManager {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn process_tx(&mut self, tx: Transaction) {
        // store transaction in the map
        self.transactions.insert(tx.tx_id, tx.clone());

        match tx.tx_type {
            TxType::Deposit => self.handle_deposit(&tx),
            TxType::Withdrawal => self.handle_withdrawal(&tx),
            TxType::Dispute => self.handle_dispute(&tx),
            TxType::Resolve => self.handle_resolve(&tx),
            TxType::ChargeBack => self.handle_chargeback(&tx),
        }
    }

    pub fn handle_deposit(&mut self, tx: &Transaction) {
        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);
        let amount = tx.amount.unwrap_or(Decimal::new(0, 4));

        // If the account is locked is not allowed to operate
        if account.locked {
            return;
        }

        account.available += amount;
        account.total += amount;
        // store the updated account
        self.accounts.insert(acc_id, account);
    }

    pub fn handle_withdrawal(&mut self, tx: &Transaction) {
        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);
        let amount = tx.amount.unwrap_or(Decimal::new(0, 4));

        // If the account is locked is not allowed to operate
        // Or the amount to withdraw is larger than the total funds
        if account.locked || account.available < amount {
            return;
        }

        account.available -= amount;
        account.total -= amount;
        // store the updated account
        self.accounts.insert(acc_id, account);
    }

    pub fn handle_dispute(&mut self, tx: &Transaction) {}

    pub fn handle_resolve(&mut self, tx: &Transaction) {}

    pub fn handle_chargeback(&mut self, tx: &Transaction) {}

    fn check_account(&mut self, acc_id: u16) -> Account {
        let account = self.accounts.get(&acc_id);
        match account {
            None => Account::new(acc_id),
            Some(acc) => acc.clone(),
        }
    }
}

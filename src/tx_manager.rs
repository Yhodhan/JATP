use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::accounts::account::Account;
use crate::transactions::transaction::Transaction;
use crate::transactions::transaction::TxType;
pub struct TxManager {
    pub accounts: HashMap<u16, Account>,
    pub transactions: HashMap<u32, Transaction>,
    pub txs_disputed: HashMap<u32, Transaction>,
}

impl TxManager {
    pub fn new() -> Self {
        TxManager {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            txs_disputed: HashMap::new(),
        }
    }

    pub fn process_tx(&mut self, tx: Transaction) {
        match tx.tx_type {
            TxType::Deposit => self.handle_deposit(&tx),
            TxType::Withdrawal => self.handle_withdrawal(&tx),
            TxType::Dispute => self.handle_dispute(&tx),
            TxType::Resolve => self.handle_resolve(&tx),
            TxType::ChargeBack => self.handle_chargeback(&tx),
        }
    }

    pub fn handle_deposit(&mut self, tx: &Transaction) {
        // store transaction in the map
        self.transactions.insert(tx.tx_id, tx.clone());

        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);

        // If Tx was missing the value then we just add 0.
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
        // store transaction in the map
        self.transactions.insert(tx.tx_id, tx.clone());

        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);

        // If Tx was missing the value then we just add 0.
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

    pub fn handle_dispute(&mut self, tx: &Transaction) {
        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);

        if account.locked {
            return;
        }

        // obtain disputed transaction
        let tx_disputed = match self.transactions.get(&tx.tx_id) {
            None => return,
            Some(t) => t,
        };

        if tx_disputed.tx_type != TxType::Deposit {
            return;
        }

        let amount = tx_disputed.amount.unwrap();

        // avoid negative available
        if account.available < amount {
            return;
        }

        account.available -= amount;
        account.held += amount;
        self.accounts.insert(acc_id, account);

        self.txs_disputed
            .insert(tx_disputed.tx_id, tx_disputed.clone());
    }

    pub fn handle_resolve(&mut self, tx: &Transaction) {
        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);

        if account.locked {
            return;
        }

        // check that it is a disputed transaction
        let tx_id = tx.tx_id;
        if self.txs_disputed.contains_key(&tx_id) {
            let tx_disputed = self.txs_disputed.get(&tx_id).unwrap();

            let amount = tx_disputed.amount.unwrap();
            account.available += amount;
            account.held -= amount;
            self.accounts.insert(acc_id, account);
            self.txs_disputed.remove(&tx_id);
        }
    }

    pub fn handle_chargeback(&mut self, tx: &Transaction) {
        let acc_id = tx.account_id;
        let mut account = self.check_account(acc_id);

        if account.locked {
            return;
        }

        let tx_id = tx.tx_id;
        if self.txs_disputed.contains_key(&tx_id) {
            // unwrap is okay since the transaction is known to be stored
            let tx_disputed = self.txs_disputed.get(&tx_id).unwrap();

            let amount = tx_disputed.amount.unwrap();
            account.held -= amount;
            account.total -= amount;
            account.locked = true;
            self.accounts.insert(acc_id, account);
            self.txs_disputed.remove(&tx_id);
        }
    }

    // check an account or create it missing
    fn check_account(&mut self, acc_id: u16) -> Account {
        let account = self.accounts.get(&acc_id);
        match account {
            None => Account::new(acc_id),
            Some(acc) => acc.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tx_manager() -> TxManager {
        let mut tx_manager = TxManager::new();

        let txs = vec![
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 1,
                amount: Some(Decimal::new(1, 4)),
            },
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 2,
                tx_id: 2,
                amount: Some(Decimal::new(2, 4)),
            },
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 3,
                amount: Some(Decimal::new(20, 4)),
            },
            Transaction {
                tx_type: TxType::Withdrawal,
                account_id: 1,
                tx_id: 4,
                amount: Some(Decimal::new(1, 4)),
            },
            Transaction {
                tx_type: TxType::Withdrawal,
                account_id: 2,
                tx_id: 5,
                amount: Some(Decimal::new(3, 4)),
            },
        ];

        for tx in txs {
            tx_manager.process_tx(tx);
        }

        tx_manager
    }

    fn setup_tx_manager_dispute_resolution() -> TxManager {
        let mut tx_manager = TxManager::new();

        let txs = vec![
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 1,
                amount: Some(Decimal::new(1, 4)),
            },
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 2,
                amount: Some(Decimal::new(2, 4)),
            },
            Transaction {
                tx_type: TxType::Dispute,
                account_id: 1,
                tx_id: 1,
                amount: None,
            },
            Transaction {
                tx_type: TxType::Resolve,
                account_id: 1,
                tx_id: 1,
                amount: None,
            },
        ];

        for tx in txs {
            tx_manager.process_tx(tx);
        }

        tx_manager
    }

    fn setup_tx_manager_dispute_chargeback() -> TxManager {
        let mut tx_manager = TxManager::new();

        let txs = vec![
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 1,
                amount: Some(Decimal::new(1, 4)),
            },
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 2,
                amount: Some(Decimal::new(2, 4)),
            },
            Transaction {
                tx_type: TxType::Dispute,
                account_id: 1,
                tx_id: 2,
                amount: None,
            },
            Transaction {
                tx_type: TxType::ChargeBack,
                account_id: 1,
                tx_id: 2,
                amount: None,
            },
        ];

        for tx in txs {
            tx_manager.process_tx(tx);
        }

        tx_manager
    }

    #[test]
    fn test_client1_available_held_total() {
        let tx_manager = setup_tx_manager();
        let account = tx_manager.accounts.get(&1).unwrap();
        assert_eq!(account.available, Decimal::new(20, 4));
        assert_eq!(account.held, Decimal::new(0, 4));
        assert_eq!(account.total, Decimal::new(20, 4));
    }

    #[test]
    fn test_client2_insufficient_funds() {
        let tx_manager = setup_tx_manager();
        let account = tx_manager.accounts.get(&2).unwrap();
        // withdrawal of 3.0 should fail, available stays at 2.0
        assert_eq!(account.available, Decimal::new(2, 4));
    }

    // ----------------------
    //     Dispute tests
    // ----------------------

    #[test]
    fn test_client_resolution() {
        let tx_manager = setup_tx_manager_dispute_resolution();
        let account = tx_manager.accounts.get(&1).unwrap();
        assert_eq!(account.available, Decimal::new(3, 4));
    }

    #[test]
    fn test_client_fail_dispute_over_withdraw() {
        let mut tx_manager = TxManager::new();
        let txs = vec![
            Transaction {
                tx_type: TxType::Deposit,
                account_id: 1,
                tx_id: 1,
                amount: Some(Decimal::new(6, 4)),
            },
            Transaction {
                tx_type: TxType::Withdrawal,
                account_id: 1,
                tx_id: 2,
                amount: Some(Decimal::new(2, 4)),
            },
            Transaction {
                tx_type: TxType::Dispute,
                account_id: 1,
                tx_id: 2,
                amount: Some(Decimal::new(2, 4)),
            },
        ];

        for tx in txs {
            tx_manager.process_tx(tx);
        }

        let account = tx_manager.accounts.get(&1).unwrap();
        assert_eq!(account.available, Decimal::new(4, 4));
        assert_eq!(account.total, Decimal::new(4, 4));
        assert_eq!(account.locked, false);
        assert_eq!(account.held, Decimal::new(0, 4));
        assert_eq!(tx_manager.txs_disputed.len(), 0);
    }

    #[test]
    fn test_client_chargeback() {
        let tx_manager = setup_tx_manager_dispute_chargeback();
        let account = tx_manager.accounts.get(&1).unwrap();
        assert_eq!(account.available, Decimal::new(1, 4));
        assert_eq!(account.total, Decimal::new(1, 4));
        assert_eq!(account.locked, true);
    }

    #[test]
    fn test_fail_transaction_after_chargeback() {
        let mut tx_manager = setup_tx_manager_dispute_chargeback();
        let extra_tx = Transaction {
            tx_type: TxType::Deposit,
            account_id: 1,
            tx_id: 3,
            amount: Some(Decimal::new(20, 4)),
        };

        tx_manager.process_tx(extra_tx);

        let account = tx_manager.accounts.get(&1).unwrap();
        assert_eq!(account.locked, true);
        assert_eq!(account.available, Decimal::new(1, 4));
    }
}

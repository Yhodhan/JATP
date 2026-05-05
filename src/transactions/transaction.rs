use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: TxType,

    #[serde(rename = "client")]
    account_id: u32,

    #[serde(rename = "tx")]
    tx_id: u32,

    amount: Option<Decimal>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
}

impl Transaction {
    pub fn new(tx_type: TxType, account_id: u32, tx_id: u32, amount: Option<Decimal>) -> Self {
        Transaction {
            tx_type,
            account_id,
            tx_id,
            amount,
        }
    }
}

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TxType,

    #[serde(rename = "client")]
    pub account_id: u16,

    #[serde(rename = "tx")]
    pub tx_id: u32,

    pub amount: Option<Decimal>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
}

impl Transaction {
    pub fn new(tx_type: TxType, account_id: u16, tx_id: u32, amount: Option<Decimal>) -> Self {
        Transaction {
            tx_type,
            account_id,
            tx_id,
            amount,
        }
    }
}

use rust_decimal::Decimal;
#[derive(Clone)]
pub struct Account {
    pub id: u16,
    pub held: Decimal,
    pub available: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Account {
            id,
            held: Decimal::new(0, 4),
            available: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
        }
    }
}

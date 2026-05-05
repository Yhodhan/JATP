use rust_decimal::Decimal;
pub struct Account {
    id: u32,
    held: Decimal,
    available: Decimal,
    total: Decimal,
    locked: bool,
}

impl Account {
    pub fn new(id: u32) -> Self {
        Account {
            id,
            held: Decimal::new(0, 4),
            available: Decimal::new(0, 4),
            total: Decimal::new(0,4),
            locked: false,
        }
    }
}

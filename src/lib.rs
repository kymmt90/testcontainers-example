use std::fmt;

use anyhow::{anyhow, Result};
use tiberius::numeric::Numeric;

#[derive(Debug, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i128,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} - {}円", self.id, self.name, self.price)
    }
}

impl Product {
    pub fn deserialize(row: &tiberius::Row) -> Result<Self> {
        let id: i32 = row.get("id").ok_or(anyhow!("id not found"))?;

        let name = row
            .get::<&str, _>("name")
            .ok_or(anyhow!("name not found"))?
            .to_string();

        let price = row
            .get::<Numeric, _>("price")
            .ok_or(anyhow!("price not found"))?
            .int_part();

        Ok(Self { id, name, price })
    }
}

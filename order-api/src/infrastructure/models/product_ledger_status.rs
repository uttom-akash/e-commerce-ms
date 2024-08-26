use crate::domain::models::product_ledger_status::ProductLedgerStatus;
use crate::schema::product_ledger_status;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = product_ledger_status)]
pub struct ProductLedgerStatusDiesel {
    pub id: i16,
    pub name: Option<String>,
}

impl Into<ProductLedgerStatus> for ProductLedgerStatusDiesel {
    fn into(self) -> ProductLedgerStatus {
        ProductLedgerStatus {
            id: self.id,
            name: self.name,
        }
    }
}

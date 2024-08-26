use crate::domain::constants::product_ledger_status::ProductLedgerStatusType;
use strum::VariantNames;

#[derive(Clone)]
pub struct ProductLedgerStatus {
    pub id: i16,
    pub name: Option<String>,
}

pub fn get_seed_product_ledger_status() -> Vec<ProductLedgerStatus> {
    ProductLedgerStatusType::VARIANTS
        .iter()
        .enumerate()
        .map(|(index, &variant)| ProductLedgerStatus {
            id: index as i16,  // Assign ID based on index
            name: Some(variant.to_string()),  // Convert enum variant to string
        })
        .collect()
}
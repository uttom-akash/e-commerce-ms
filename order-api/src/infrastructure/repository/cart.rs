use crate::domain::error::RepositoryError;
use crate::domain::models::cart::Cart;
use crate::domain::models::product_ledger::ProductLedger;
use crate::domain::repository::cart::CartRepository;
use crate::infrastructure::database::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::cart::{CartDiesel, NewCartDiesel};
use crate::infrastructure::models::product_ledger::{NewProductLedgerDiesel, ProductLedgerDiesel};
use crate::infrastructure::models::product_ledger_status::ProductLedgerStatusDiesel;
use actix_threadpool::{run, BlockingError};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

pub struct CartRepositoryImpl {
    pub pool: Arc<DBConn>,
}

impl CartRepositoryImpl {
    pub fn new(pool: Arc<DBConn>) -> Self {
        CartRepositoryImpl { pool }
    }
}

#[async_trait]
impl CartRepository for CartRepositoryImpl {
    async fn add(&self, product: ProductLedger) -> Result<Cart, RepositoryError> {
        use crate::schema::cart::dsl::cart;
        use crate::schema::cart::cart_id as cart_cart_id;
        use crate::schema::product_ledger::dsl::product_ledger;
        use crate::schema::product_ledger::{cart_id as product_ledger_cart_id, modified_at, product_id};

        let new_product_diesel = NewProductLedgerDiesel::from(product.clone());
        let mut conn = self.pool.get().unwrap();

        let result: CartDiesel = run(move || {
            let added_cart: CartDiesel = diesel::insert_into(cart)
                .values(NewCartDiesel {
                    cart_id: new_product_diesel.cart_id,
                })
                .on_conflict(cart_cart_id)
                .do_update()
                .set(cart_cart_id.eq(new_product_diesel.cart_id))
                .get_result(&mut conn)?;

            let _product_ledger_diesel: ProductLedgerDiesel = diesel::insert_into(product_ledger)
                .values(new_product_diesel)
                .on_conflict((product_ledger_cart_id, product_id)) // Conflict target
                .do_update()
                .set((
                    modified_at.eq(diesel::dsl::now), // Update timestamp
                ))
                .get_result(&mut conn)?;

            Ok(added_cart)
        })
            .await
            .map_err(|e: BlockingError<Error>| DieselRepositoryError::from(e).into_inner())?;


        Ok(result.into())
    }

    async fn list(&self, cart_uuid: Uuid) -> Result<Vec<ProductLedger>, RepositoryError> {
        use crate::schema::product_ledger::dsl::product_ledger;
        use crate::schema::product_ledger::{cart_id as product_ledger_cart_id, cart_status_id};
        use crate::schema::product_ledger_status::dsl::product_ledger_status;
        use crate::schema::product_ledger_status::id as product_ledger_status_id;

        let mut conn = self.pool.get().unwrap();

        let result = run(move || {
            product_ledger
                .inner_join(product_ledger_status.on(cart_status_id.assume_not_null().eq(product_ledger_status_id)))
                .filter(product_ledger_cart_id.eq(cart_uuid))
                .load::<(ProductLedgerDiesel, ProductLedgerStatusDiesel)>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        let items = result.into_iter().map(|(cart_diesel, cart_status_diesel)| {
            let mut product_ledger_model: ProductLedger = cart_diesel.into();

            product_ledger_model.product_ledger_status = Some(cart_status_diesel.into());

            product_ledger_model
        }).collect();

        Ok(items)
    }
}

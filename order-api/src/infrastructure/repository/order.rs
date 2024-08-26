use crate::domain::error::RepositoryError;
use crate::domain::models::order::Order;
use crate::domain::repository::order::OrderRepository;
use crate::infrastructure::database::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::cart::CartDiesel;
use crate::infrastructure::models::orders::{NewOrderDiesel, OrderDiesel};
use crate::infrastructure::models::product_ledger::ProductLedgerDiesel;
use actix_threadpool::{run, BlockingError};
use async_trait::async_trait;
use diesel::result::Error;
use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

pub struct OrderRepositoryImpl {
    pub pool: Arc<DBConn>,
}

impl OrderRepositoryImpl {
    pub fn new(pool: Arc<DBConn>) -> Self {
        OrderRepositoryImpl { pool }
    }
}

#[async_trait]
impl OrderRepository for OrderRepositoryImpl {
    async fn add(&self, new_order: Order) -> Result<Order, RepositoryError> {
        use crate::schema::orders::dsl::orders;
        use crate::schema::cart::dsl::cart;
        use crate::schema::cart::cart_id;
        use crate::schema::cart::order_id;

        let new_order_diesel: NewOrderDiesel = NewOrderDiesel::from(new_order);

        let mut conn = self.pool.get().unwrap();

        let added_order: Order = run(move || {
            let added_order_diesel: OrderDiesel = diesel::insert_into(orders)
                .values(new_order_diesel.clone())
                .get_result(&mut conn)?;

            let updated_cart_diesel: CartDiesel = diesel::update(cart)
                .filter(cart_id.eq(new_order_diesel.cart_id))
                .set(order_id.eq(new_order_diesel.order_id))
                .get_result(&mut conn)?;

            let mut added_order: Order = added_order_diesel.into();
            added_order.cart = Some(updated_cart_diesel.into());

            Ok(added_order)
        })
            .await
            .map_err(|v: BlockingError<Error>| DieselRepositoryError::from(v).into_inner())?;

        Ok(added_order)
    }

    async fn get(&self, filter_order_id: Uuid) -> Result<Order, RepositoryError> {
        use crate::schema::orders::dsl::{order_id as order_order_id, orders};
        use crate::schema::cart::dsl::{cart, cart_id as cart_cart_id, order_id as cart_order_id};
        use crate::schema::product_ledger::dsl::{cart_id as product_ledger_cart_id, product_ledger};

        let mut conn = self.pool.get().unwrap();

        // Execute the query
        let results = run(move || {
            orders
                .inner_join(cart.on(cart_order_id.assume_not_null().eq(order_order_id)))
                .inner_join(product_ledger.on(product_ledger_cart_id.eq(cart_cart_id)))
                .filter(order_order_id.eq(filter_order_id))
                .load::<(OrderDiesel, CartDiesel, ProductLedgerDiesel)>(&mut conn)
        }).await
            .map_err(|v: BlockingError<Error>| DieselRepositoryError::from(v).into_inner())?;

        let mut order_opt: Option<OrderDiesel> = None;
        let mut cart_opt: Option<CartDiesel> = None;
        let mut product_ledgers: Vec<ProductLedgerDiesel> = Vec::new();

        for (order_diesel, cart_diesel, ledger_diesel) in results {
            if order_opt.is_none() {
                order_opt = Some(order_diesel);
            }
            if cart_opt.is_none() {
                cart_opt = Some(cart_diesel);
            }
            product_ledgers.push(ledger_diesel);
        }

        let mut order_domain_model: Order = order_opt.unwrap().into();
        order_domain_model.cart = Some(cart_opt.unwrap().into());

        if let Some(cart_domain_model) = &mut order_domain_model.cart {
            cart_domain_model.product_ledger = product_ledgers.into_iter().map(|v| v.into()).collect();
        }

        Ok(order_domain_model)
    }
}
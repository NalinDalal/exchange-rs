use crate::{models::Order, DbPool};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrderRepository {
    pool: DbPool,
}

impl OrderRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        side: &str,
        price: Decimal,
        qty: Decimal,
    ) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO "Order" (id, "userId", side, price, qty, status, "createdAt")
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(side)
        .bind(price)
        .bind(qty)
        .bind("open")
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(order)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<Order>, sqlx::Error> {
        let order = sqlx::query_as::<_, Order>(
            r#"SELECT * FROM "Order" WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(order)
    }

    pub async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Order>, sqlx::Error> {
        let orders = sqlx::query_as::<_, Order>(
            r#"SELECT * FROM "Order" WHERE "userId" = $1 ORDER BY "createdAt" DESC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(orders)
    }

    pub async fn find_open_orders(&self) -> Result<Vec<Order>, sqlx::Error> {
        let orders = sqlx::query_as::<_, Order>(
            r#"SELECT * FROM "Order" WHERE status = 'open' ORDER BY "createdAt" ASC"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(orders)
    }

    pub async fn update_status(&self, id: &Uuid, status: &str) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            UPDATE "Order" 
            SET status = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(order)
    }

    pub async fn cancel_order(&self, id: &Uuid, user_id: &Uuid) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            UPDATE "Order" 
            SET status = 'cancelled'
            WHERE id = $1 AND "userId" = $2 AND status = 'open'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(order)
    }
}

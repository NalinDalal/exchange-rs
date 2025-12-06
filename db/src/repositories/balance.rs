use crate::{models::Balance, DbPool};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone)]
pub struct BalanceRepository {
    pool: DbPool,
}

impl BalanceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        asset: &str,
        total: Decimal,
    ) -> Result<Balance, sqlx::Error> {
        let balance = sqlx::query_as::<_, Balance>(
            r#"
            INSERT INTO "Balance" (id, asset, total, reserved, "userId", "updatedAt")
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(asset)
        .bind(total)
        .bind(Decimal::ZERO)
        .bind(user_id)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(balance)
    }

    pub async fn find_by_user_and_asset(
        &self,
        user_id: &Uuid,
        asset: &str,
    ) -> Result<Option<Balance>, sqlx::Error> {
        let balance = sqlx::query_as::<_, Balance>(
            r#"SELECT * FROM "Balance" WHERE "userId" = $1 AND asset = $2"#
        )
        .bind(user_id)
        .bind(asset)
        .fetch_optional(&self.pool)
        .await?;

        Ok(balance)
    }

    pub async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Balance>, sqlx::Error> {
        let balances = sqlx::query_as::<_, Balance>(
            r#"SELECT * FROM "Balance" WHERE "userId" = $1"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(balances)
    }

    pub async fn update_balance(
        &self,
        id: &Uuid,
        total: Decimal,
        reserved: Decimal,
    ) -> Result<Balance, sqlx::Error> {
        let balance = sqlx::query_as::<_, Balance>(
            r#"
            UPDATE "Balance" 
            SET total = $1, reserved = $2, "updatedAt" = $3
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(total)
        .bind(reserved)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(balance)
    }
}

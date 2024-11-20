use sqlx::{query, query_as, query_scalar, SqlitePool};

use crate::model::{Commission, PortfolioCategory, PortfolioItem};

pub struct CommissionInsertData {
    pub title: String,
    pub contact: String,
    pub description: String,
    pub status: String
}

pub struct PortfolioInsertData {
    pub category: PortfolioCategory,
    pub title: String,
    pub image_paths: Vec<String>,
}

struct PortfolioItemBase {
    id: i64,
    category: PortfolioCategory,
    title: String,
}


pub async fn get_sqlite_pool() -> Result<SqlitePool, sqlx::Error>{
    SqlitePool::connect("sqlite://commissions.sqlite3").await
}

#[derive(Clone)]
pub struct CommissionRepository {
    pool: SqlitePool,
}

impl CommissionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Commission>,sqlx::Error> {
        query_as!(Commission, "SELECT * FROM commissions").fetch_all(&self.pool).await
    }

    pub async fn insert(&self, data: CommissionInsertData ) -> Result<(), sqlx::Error>{
        query!("
                INSERT INTO commissions(
                    title,
                    contact,
                    description,
                    status
                )
                VALUES (?, ?, ?, ?)",
            data.title,
            data.contact,
            data.description,
            data.status
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<(), sqlx::Error> {
        query!(" DELETE FROM commissions WHERE id = ?", id).execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct PortfolioRepository {
    pool: SqlitePool,
}

impl PortfolioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<PortfolioItem>, sqlx::Error> {
        let items = query_as!(PortfolioItemBase, r#"
            select
                id,
                category_id as "category: _",
                title
            from
                portfolio_items
            "#)
            .fetch_all(&self.pool)
            .await?;

        let items = futures::future::join_all(
            items.into_iter().map(|item| self.portfolio_itembase_to_item(item)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(items)
    }

    async fn portfolio_itembase_to_item(
        &self,
        item: PortfolioItemBase
    ) -> Result<PortfolioItem, sqlx::Error> {
        let images = query_scalar!("
                select
                    path
                from
                    portfolio_images
                where
                    item_id = ?
            ",
            item.id)
            .fetch_all(&self.pool)
        .await?;

        Ok(PortfolioItem {
            id: item.id,
            category: item.category,
            title: item.title,
            image_paths: images,
        })
    }

    pub async fn insert(&self, data: PortfolioInsertData) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let id = query_scalar!("
                INSERT INTO portfolio_items(
                    category_id,
                    title
                )
                VALUES (?, ?)
                RETURNING id",
            data.category,
            data.title,
        )
        .fetch_one(&mut *transaction)
        .await?;

        for path in data.image_paths {
            query!(
                "INSERT INTO portfolio_images(
                    item_id,
                    path
                )
                VALUES(?, ?)",
                id,
                path
                )
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await
    }
}


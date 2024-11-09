use sqlx::{SqlitePool, query_as, query};

#[derive(Debug)]
pub struct Commission{
    pub id: i64,
    pub title: String,
    pub contact: String,
    pub description: String,
    pub status: String,
}

pub struct CommissionInsertData{
    pub title: String,
    pub contact: String,
    pub description: String,
    pub status: String
}

pub async fn get_sqlite_pool () -> Result<SqlitePool, sqlx::Error>{
    SqlitePool::connect("sqlite://commissions.sqlite3").await
}

pub async fn get_commissions(pool: &SqlitePool) -> Result<Vec<Commission>,sqlx::Error> {
    query_as!(Commission, "SELECT * FROM commissions").fetch_all(pool).await
}

pub async fn insert_commission(pool: &SqlitePool, data: CommissionInsertData ) -> Result<(), sqlx::Error>{
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
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_commission_by_id(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    query!(" DELETE FROM commissions WHERE id = ?", id).execute(pool).await?;
    Ok(())
}

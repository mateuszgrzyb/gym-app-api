use sqlx::{query_file_as, FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
pub struct Ad {
    pub id: i32,
    pub key: Uuid,
}

impl Ad {
    pub async fn create(db: &PgPool) -> sqlx::Result<Self> {
        query_file_as!(Self, "sql/ad/create.sql", Uuid::new_v4(),)
            .fetch_one(db)
            .await
    }

    pub async fn get_by_key(key: &Uuid, db: &PgPool) -> sqlx::Result<Option<Self>> {
        query_file_as!(Self, "sql/ad/get_by_key.sql", key,)
            .fetch_optional(db)
            .await
    }

    pub async fn delete(&self, db: &PgPool) -> sqlx::Result<Self> {
        query_file_as!(Self, "sql/ad/delete.sql", self.id)
            .fetch_one(db)
            .await
    }
}

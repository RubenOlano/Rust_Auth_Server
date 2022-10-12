use color_eyre::Result;
use sqlx::{postgres::PgQueryResult, PgPool};
use std::sync::Arc;

use crate::{
    config::crypto::CryptoService,
    models::user::{NewUser, User},
};

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser, crypto_service: &CryptoService) -> Result<User> {
        let pass_hash = crypto_service.hash_password(new_user.password).await?;
        let user = sqlx::query_as::<_, User>(
            r#" 
            insert into users (fname, lname, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(new_user.fname)
        .bind(new_user.lname)
        .bind(new_user.email)
        .bind(pass_hash)
        .fetch_one(&*self.pool)
        .await?;
        Ok(user)
    }
}

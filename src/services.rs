use async_trait::async_trait;
use uuid::Uuid;

use crate::{api::AppData, env_vars::ENV_VARS, errors::R, models::Ad};

#[async_trait]
pub trait Service {
    type O;

    async fn run(self, data: &AppData) -> R<Self::O>;
}

#[derive(PartialEq, Eq)]
pub struct GenerateKey<'input> {
    pub username: &'input str,
    pub password: &'input str,
}

#[async_trait]
impl<'input> Service for GenerateKey<'input> {
    type O = Option<Uuid>;

    async fn run(self, data: &AppData) -> R<Self::O> {
        let expected_data = Self {
            username: &ENV_VARS.username,
            password: &ENV_VARS.password,
        };

        if self != expected_data {
            return Ok(None);
        }

        let key = Ad::create(&data.db).await?.key;

        Ok(Some(key))
    }
}

pub struct ValidateKey {
    pub key: Uuid,
}

#[async_trait]
impl Service for ValidateKey {
    type O = bool;

    async fn run(self, data: &AppData) -> R<Self::O> {
        let Some(ad) = Ad::get_by_key(&self.key, &data.db).await? else {
            return Ok(false)
        };

        ad.delete(&data.db).await?;

        Ok(true)
    }
}

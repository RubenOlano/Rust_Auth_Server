use argonautica::Hasher;
use color_eyre::Result;
use futures::compat::Future01CompatExt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    #[instrument(skip(self, password))]
    pub async fn hash_password(&self, password: String) -> String {
        Hasher::default()
            .with_password(password)
            .with_secret_key(self.key.as_str())
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|e| eyre!("Error hashing password: {:?}", e))
    }
}

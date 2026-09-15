use crate::services::secrets::load_secrets;
use crate::util::config::MergeOptionalConfig;
use serde::de::DeserializeOwned;

pub async fn config_from_secret<A, OA>(secret_name: String) -> A
where
    A: DeserializeOwned + From<OA>,
    OA: DeserializeOwned + MergeOptionalConfig,
{
    config_from_secrets(vec![secret_name]).await
}

pub async fn config_from_secrets<A, OA>(secret_names: Vec<String>) -> A
where
    A: DeserializeOwned + From<OA>,
    OA: DeserializeOwned + MergeOptionalConfig,
{
    let config_from_env = envy::from_env::<OA>().unwrap();

    let secret_config = crate::Config::new().await;
    let config_from_secret = load_secrets::<OA>(&secret_config, secret_names).await;
    let config_from_secret = config_from_secret.unwrap();

    config_from_env.merge(config_from_secret).into()
}

pub trait MergeOptionalConfig {
    fn merge(self, s: Self) -> Self;
}

pub struct Config {
    pub(crate) client: aws_sdk_secretsmanager::Client,
}

impl Config {
    pub async fn new() -> Config {
        let config = aws_config::load_from_env().await;

        Self {
            client: aws_sdk_secretsmanager::Client::new(&config),
        }
    }
}
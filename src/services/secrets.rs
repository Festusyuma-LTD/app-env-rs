use crate::util::config::Config;
use anyhow::Error;
use serde::Deserialize;
use shared::response::ServiceResult;

fn merge_json_objects(a: &mut serde_json::Value, b: serde_json::Value) {
    if let (serde_json::Value::Object(a_map), serde_json::Value::Object(b_map)) = (a, b) {
        for (k, v) in b_map {
            a_map.insert(k, v);
        }
    }
}

fn parse_secrets(secrets_string: Option<String>) -> ServiceResult<serde_json::Value> {
    let secrets_string = secrets_string.unwrap_or(String::from(r#"{}"#));
    let secrets_value = serde_json::from_str(&secrets_string).map_err(Error::from)?;

    match secrets_value {
        serde_json::Value::Object(map) => Ok(serde_json::Value::Object(
            map.into_iter()
                .map(|(k, v)| (k.to_lowercase(), v))
                .collect(),
        )),
        other => Ok(other),
    }
}

pub async fn load_secret<T: for<'a> Deserialize<'a>>(
    config: &Config,
    secret_name: String,
) -> ServiceResult<T> {
    load_secrets(config, vec![secret_name]).await
}

pub async fn load_secrets<T: for<'a> Deserialize<'a>>(
    config: &Config,
    secret_names: Vec<String>,
) -> ServiceResult<T> {
    let mut all_loaded_secrets: serde_json::Value =
        serde_json::from_str(r#"{}"#).map_err(Error::from)?;

    for secret_name in secret_names {
        let loaded_secrets = config
            .client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await
            .map_err(Error::from)?
            .secret_string;

        let loaded_secrets = parse_secrets(loaded_secrets)?;

        merge_json_objects(&mut all_loaded_secrets, loaded_secrets);
    }

    Ok(serde_json::from_value(all_loaded_secrets).map_err(Error::from)?)
}

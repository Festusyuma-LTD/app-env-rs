# env

Library crate for building an app's config from two layers: plain environment variables,
and secrets pulled from AWS Secrets Manager — merged together into one strongly-typed
struct.

## What it does

- **`config_from_secret`** / **`config_from_secrets`**: loads an "optional" config shape
  from the process environment (via `envy`), loads the named secret(s) from Secrets
  Manager, merges the two (env values winning over secret values), and converts the
  result into the final config type.
- **`load_secret`** / **`load_secrets`**: the lower-level piece — fetches one or more
  named secrets, parses each `SecretString` as JSON, lower-cases its top-level keys, and
  merges them into a single JSON object before deserializing it into `T`.

## Wiring it into an app

Define two structs: an "optional" one whose fields mirror the env vars/secret keys
(all `Option<_>`, implementing `MergeOptionalConfig`), and the final one your app
actually uses (implementing `From<Optional>`, where required fields are unwrapped and
optional ones are validated):

```rust
use env::MergeOptionalConfig;
use serde::Deserialize;

#[derive(Deserialize)]
struct OptionalAppConfig {
    port: Option<String>,
    api_key: Option<String>,
}

impl MergeOptionalConfig for OptionalAppConfig {
    fn merge(self, right: Self) -> Self {
        Self {
            port: self.port.or(right.port),
            api_key: self.api_key.or(right.api_key),
        }
    }
}

struct AppConfig {
    port: u16,
    api_key: String,
}

impl From<OptionalAppConfig> for AppConfig {
    fn from(value: OptionalAppConfig) -> Self {
        use env::messages::*;

        Self {
            port: value.port.unwrap_or("3000".into()).parse().expect(&invalid("port")),
            api_key: value.api_key.expect(&required("api_key")),
        }
    }
}

let config = env::config_from_secret::<AppConfig, OptionalAppConfig>("my-app/secret".into()).await;
```

`config_from_secret`/`config_from_secrets` panic (`.unwrap()`) if the env layer fails to
parse or the secret fetch/parse fails — there's no recoverable path today, so call this
during startup, before anything depends on the config existing.

## `env::messages`

Two small helpers for building `From<Optional>` panic messages: `required(key)` →
`"KEY is required"`, `invalid(key)` → `"invalid value for KEY"` (both upper-casing the
key).

## Config (Secrets Manager client)

`Config::new()` builds the `aws-sdk-secretsmanager` client via
`aws_config::load_from_env()`, so it needs AWS credentials/region available the usual
way (env vars, instance profile, etc.) — same as the other services in this family.

pub fn required(key: &str) -> String {
    format!("{} is required", key.to_uppercase())
}

pub fn invalid(key: &str) -> String {
    format!("invalid value for {}", key.to_uppercase())
}

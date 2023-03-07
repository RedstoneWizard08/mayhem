pub fn make_env_key(prefix: &str, name: &str) -> String {
    return format!("{}_{}", prefix, name);
}

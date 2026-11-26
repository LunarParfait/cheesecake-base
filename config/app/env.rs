use crate::cheesecake::env_utils::{AppEnvironmentTrait, EnvLock};

/// Application environment variables
pub struct AppEnv {}

pub static ENV: EnvLock<AppEnv> = EnvLock::new();

impl AppEnvironmentTrait for AppEnv {
    fn new() -> Self {
        Self {}
    }
}

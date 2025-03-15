pub use auth::*;

mod auth;

#[derive(Debug, Clone)]
pub struct V2 {
    pub oauth: TokenService,
}

impl V2 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            oauth: TokenService::new(config),
        }
    }
}
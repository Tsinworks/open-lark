pub mod v1;
pub mod v2;

#[derive(Debug, Clone)]
pub struct AuthenService {
    pub v1: v1::V1,
    pub v2: v2::V2,
}

impl AuthenService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            v1: v1::V1::new(config.clone()),
            v2: v2::V2::new(config),
        }
    }
}

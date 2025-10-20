use crate::{cache::service::CacheService, db::repositories::StringRepository};

#[derive(Clone)]
pub struct AppState {
    pub repository: StringRepository,
    pub cache: CacheService,
}

use crate::db::repositories::StringRepository;

#[derive(Clone)]
pub struct AppState {
    pub repository: StringRepository,
}

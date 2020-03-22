use {mongodb::Client, std::sync::Arc};

pub struct AppState {
    pub db_client: Arc<Client>,
}

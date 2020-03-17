use {
    mongodb::Client, 
    std::sync::Arc,
};

// #[derive(Copy)]
pub struct AppState {
    pub db_client: Arc<Client>,
}

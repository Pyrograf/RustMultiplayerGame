use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use database_adapter::DatabaseAdapter;

pub struct AppData {
    pub database_adapter: Arc<dyn DatabaseAdapter>,
}

impl AppData {
    pub fn new(database_adapter: Arc<dyn DatabaseAdapter>) -> Self {
        Self {
            database_adapter,
        }
    }
}

impl Debug for AppData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppData")
    }
}
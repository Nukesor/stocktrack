use diesel::{
    expression_methods::ExpressionMethods,
    query_dsl::QueryDsl,
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl,
};
use std::sync::Arc;

use crate::models::{TodoItem, TodoList};

#[derive(Clone)]
pub struct DbClient {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DbClient {
    pub fn new(db_url: &str) -> Self {
        Self {
            db_pool: Arc::new(
                Pool::builder()
                    .build(ConnectionManager::<PgConnection>::new(db_url))
                    .expect("Failed to create pool."),
            ),
        }
    }
}

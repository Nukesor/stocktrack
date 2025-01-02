use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    username: String,
    task: String,
    complete: bool,
}

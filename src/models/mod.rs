use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct TodoInput {
    pub title: String,
}
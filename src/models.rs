use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    #[allow(dead_code)]
    id: i32,
    pub client_id: String,
    pub client_type: String,
    pub name: String,
    pub description: String,
    pub email: String,
    pub website: String,
}

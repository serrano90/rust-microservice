use crate::schema::resorts;

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "resorts"]
pub struct NewResort {
    pub name: String,
}

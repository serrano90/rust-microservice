use diesel::QueryDsl;
/**
 * Resort Repository ORM
*/
use diesel::RunQueryDsl;

use crate::domain::dto::resort::ResortDTO;
use crate::domain::dto::resort::ResortDTOList;
use crate::domain::entity::resort::Resort;
use crate::domain::repository::resort_repository::ResortRepository;
use crate::infrastructure::models::resort::NewResort;
use crate::infrastructure::repository::connection::ConnectionManager;
use crate::schema::resorts;
use crate::schema::resorts::dsl::resorts as resorts_list;

pub struct ORMResortRepository {
    db: ConnectionManager,
}

impl ORMResortRepository {
    pub fn new(db: ConnectionManager) -> Self {
        ORMResortRepository { db: db }
    }
}

impl ResortRepository for ORMResortRepository {
    fn all(&self) -> ResortDTOList {
        let conn = &self.db.conn;

        let results = resorts_list
            .load::<ResortDTO>(conn)
            .expect("Error loading resorts");

        ResortDTOList(results)
    }

    fn find(&self, id: i32) -> ResortDTO {
        let conn = &self.db.conn;

        let result = resorts::table
            .find(id)
            .first(conn)
            .expect("Error loading resorts");

        result
    }

    fn delete(&self, id: i32) -> bool {
        false
    }

    fn update(&self, resort: Resort) -> Result<(), diesel::result::Error> {
        let conn = &self.db.conn;

        let new_resort = NewResort {
            name: resort.get_name(),
        };

        diesel::update(resorts::dsl::resorts.find(resort.get_id()))
            .set(new_resort)
            .execute(conn)?;

        Ok(())
    }

    fn create(&self, resort: Resort) -> ResortDTO {
        let conn = &self.db.conn;

        let new_resort = NewResort {
            name: resort.get_name(),
        };

        let result = diesel::insert_into(resorts::table)
            .values(&new_resort)
            .get_result::<ResortDTO>(conn)
            .expect("Error saving Resort");

        result
    }
}

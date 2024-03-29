pub use sea_orm_migration::prelude::*;

mod m20230912_062515_create_tasks_table;
mod m20230913_033900_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230912_062515_create_tasks_table::Migration),
            Box::new(m20230913_033900_create_users_table::Migration),
        ]
    }
}

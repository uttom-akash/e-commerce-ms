use crate::schema::cart_status;
use crate::schema::diesel_schema_migrations;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn seed_db() {
    let migration_exists = diesel_schema_migrations::table
        .filter(diesel_schema_migrations::version.eq(migration_name))
        .first::<(String,)>(conn)
        .optional()?
        .is_some();

    if !migration_exists {
        // Insert data into cart_status
        let new_statuses = vec![
            CartStatusInsert { id: 0, name: Some("Pending".to_string()) },
            CartStatusInsert { id: 1, name: Some("Confirmed".to_string()) },
            // Add other statuses as needed
        ];

        diesel::insert_into(cart_status::table)
            .values(&new_statuses)
            .execute(conn)?;

        // Insert migration record into __diesel_schema_migrations
        let migration_record = MigrationRecordInsert {
            version: migration_name.to_string(),
            name: Some(migration_name.to_string()),
            run_on: Some(Utc::now().naive_utc()),
        };

        diesel::insert_into(diesel_schema_migrations::table)
            .values(&migration_record)
            .execute(conn)?;
    }
}
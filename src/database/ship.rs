use chrono::{DateTime, Utc};
use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, UserMarker}};

pub struct Ship {
    pub guild_id: Id<GuildMarker>,
    pub combined_ids: String,
    pub name: String,
    pub created_at: DateTime<Utc>
}

impl From<Row> for Ship {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            combined_ids: row.get(1),
            name: row.get(2),
            created_at: row.get::<_, DateTime<Utc>>(3)
        }
    }
}

impl Database {
    pub async fn create_ship(&self, guild_id: Id<GuildMarker>, id_one: Id<UserMarker>, id_two: Id<UserMarker>) {
        let client = self.get_object().await;
        let query = "INSERT INTO ship(guild_id, combined_ids) VALUES($1, $2);";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &format!("{}-{}", id_one.to_string(), id_two.to_string())
            ]
        ).await.unwrap();
    }

    pub async fn read_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Option<Ship> {
        let client = self.get_object().await;
        let query = "SELECT * FROM ship WHERE guild_id = $1 AND combined_ids LIKE $2;";

        match client.query_one(
            query,
            &[
                &(guild_id.get() as i64),
                &format!("%{}%", user_id.to_string())
            ]
        ).await {
            Ok(row) => Some(row.into()),
            Err(_) => None,
        }        
    }

    pub async fn update_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>, name: String) {
        let client = self.get_object().await;
        let query = "UPDATE ship SET name = $3 WHERE guild_id = $1 AND combined_ids LIKE $2;";

        client.query(query, &[&(guild_id.get() as i64), &format!("%{}%", user_id.to_string()), &name]).await.unwrap(); 
    }

    pub async fn delete_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM ship WHERE guild_id = $1 AND combined_ids LIKE $2;";

        client.query(query, &[&(guild_id.get() as i64), &format!("%{}%", user_id.to_string())]).await.unwrap();

    }
}
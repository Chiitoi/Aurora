use chrono::{DateTime, Utc};
use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, UserMarker}};

pub struct Ship {
    pub guild_id: Id<GuildMarker>,
    pub id_one: Id<UserMarker>,
    pub id_two: Id<UserMarker>,
    pub name: String,
    pub created_at: DateTime<Utc>
}

impl From<Row> for Ship {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            id_one: Id::new(row.get::<_, i64>(1) as u64),
            id_two: Id::new(row.get::<_, i64>(2) as u64),
            name: row.get(3),
            created_at: row.get::<_, DateTime<Utc>>(4)
        }
    }
}

impl Database {
    pub async fn create_ship(&self, guild_id: Id<GuildMarker>, id_one: Id<UserMarker>, id_two: Id<UserMarker>) {
        let client = self.get_object().await;
        let query = "INSERT INTO ship(guild_id, id_one, id_two) VALUES($1, $2, $3) ON CONFLICT DO NOTHING;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(id_one.get() as i64),
                &(id_two.get() as i64)
            ]
        ).await.unwrap();
    }

    pub async fn read_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Option<Ship> {
        let client = self.get_object().await;
        let query = "SELECT * FROM ship WHERE guild_id = $1 AND (id_one = $2 OR id_two = $2);";

        match client.query_one(query, &[&(guild_id.get() as i64), &(user_id.get() as i64)]).await {
            Ok(row) => Some(row.into()),
            Err(_) => None,
        }        
    }

    pub async fn update_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>, name: String) {
        let client = self.get_object().await;
        let query = "UPDATE ship SET name = $3 WHERE guild_id = $1 AND (id_one = $2 OR id_two = $2);";

        client.query(query, &[&(guild_id.get() as i64), &(user_id.get() as i64), &name]).await.unwrap(); 
    }

    pub async fn delete_ship(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM ship WHERE guild_id = $1 AND (id_one = $2 OR id_two = $2);";

        client.query(query, &[&(guild_id.get() as i64), &(user_id.get() as i64)]).await.unwrap();
    }
}
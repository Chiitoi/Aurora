use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, RoleMarker, UserMarker}};

pub struct SharedRole {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
    pub owner_ids: Vec<Id<UserMarker>>
}

impl From<Row> for SharedRole {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            role_id: Id::new(row.get::<_, i64>(1) as u64),
            owner_ids: row.get::<_, Vec<i64>>(2).into_iter().map(|id| Id::new(id as u64)).collect()
        }
    }
}

impl Database {
    pub async fn create_shared_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>, owner_ids: Vec<Id<UserMarker>>) {
        let client = self.get_object().await;
        let query = "INSERT INTO shared_role(guild_id, role_id, owner_ids) VALUES($1, $2, $3) ON CONFLICT DO NOTHING;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(role_id.get() as i64),
                &owner_ids.into_iter().map(|id| id.get() as i64).collect::<Vec<i64>>()
            ]
        ).await.unwrap();
    }

    pub async fn delete_shared_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM shared_role WHERE guild_id = $1 AND role_id = $2;";

        client.query(query, &[&(guild_id.get() as i64), &(role_id.get() as i64)]).await.unwrap();
    }

    pub async fn delete_shared_roles(&self, guild_id: Id<GuildMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM shared_role WHERE guild_id = $1;";

        client.query(query, &[&(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn read_shared_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Option<SharedRole> {
        let client = self.get_object().await;
        let query = "SELECT * FROM shared_role WHERE guild_id = $1 AND role_id = $2;";

        match client.query_one(query, &[&(guild_id.get() as i64), &(role_id.get() as i64)]).await {
            Ok(row) => Some(row.into()),
            Err(_) => None
        }
    }

    pub async fn read_shared_roles(&self, guild_id: Id<GuildMarker>) -> Option<Vec<SharedRole>> {
        let client = self.get_object().await;
        let query = "SELECT * FROM shared_role WHERE guild_id = $1 ORDER BY role_id;";

        match client.query(query, &[&(guild_id.get() as i64)]).await {
            Ok(rows) => {
                let mut shared_roles = Vec::new();

                for row in rows.into_iter() {
                    shared_roles.push(SharedRole::from(row));
                }

                Some(shared_roles)
            },
            Err(_) => None
        }
    }

    pub async fn update_owner_ids(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>, owner_ids: Vec<Id<UserMarker>>) {
        let client = self.get_object().await;
        let query = "UPDATE shared_role SET owner_ids = $3 WHERE guild_id = $1 AND role_id = $2;";

        client.query(
            query,
            &[
                &owner_ids.into_iter().map(|id| id.get() as i64).collect::<Vec<i64>>(),
                &(guild_id.get() as i64),
                &(role_id.get() as i64)                
            ]
        ).await.unwrap();
    }
}
use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, RoleMarker}};

pub enum RoleKind {
    Message,
    Voice    
}

pub struct LevelRole {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
    pub kind: RoleKind,
    pub level: u16,
    pub is_persistent: bool
}

impl RoleKind {
    fn as_str(&self) -> &'static str {
        match self {
            RoleKind::Message => "message",
            RoleKind::Voice => "voice",
        }
    }
}

impl From<Row> for LevelRole {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            role_id: Id::new(row.get::<_, i64>(1) as u64),
            kind: match row.get::<_, String>(2).as_str() {
                "message" => RoleKind::Message,
                _ => RoleKind::Voice
            },
            level: row.get::<_, i16>(3) as u16,
            is_persistent: row.get(4),  
        }
    }
}

impl Database {
    pub async fn create_level_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>, kind: RoleKind, level: u16, is_persistent: bool) {
        let client = self.get_object().await;
        let query = "INSERT INTO level_role(guild_id, role_id, kind, level, is_persistent) VALUES($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(role_id.get() as i64),
                &(kind.as_str()),
                &(level as i16),
                &is_persistent
            ]
        ).await.unwrap();
    }

    pub async fn delete_level_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM level_role WHERE guild_id = $1 AND role_id = $2;";

        client.query(query, &[&(guild_id.get() as i64), &(role_id.get() as i64)]).await.unwrap();
    }

    pub async fn delete_level_roles(&self, guild_id: Id<GuildMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM level_role WHERE guild_id = $1;";

        client.query(query, &[&(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn read_level_role(&self, guild_id: Id<GuildMarker>, kind: RoleKind, level: u16) -> Option<LevelRole> {
        let client = self.get_object().await;
        let query = "SELECT * FROM level_role WHERE guild_id = $1 AND kind = $2 AND level = $3;";

        match client.query_one(query, &[&(guild_id.get() as i64), &(kind.as_str()), &(level as i16)]).await {
            Ok(row) => Some(row.into()),
            Err(_) => None
        }
    }

    pub async fn read_level_roles(&self, guild_id: Id<GuildMarker>) -> Option<Vec<LevelRole>> {
        let client = self.get_object().await;
        let query = "SELECT * FROM level_role WHERE guild_id = $1;";

        match client.query(query, &[&(guild_id.get() as i64)]).await {
            Ok(rows) => {
                let mut level_roles = Vec::new();

                for row in rows.into_iter() {
                    level_roles.push(LevelRole::from(row));
                }

                Some(level_roles)
            },
            Err(_) => None
        }
    }
}
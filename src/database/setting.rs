use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, UserMarker}};

pub enum Module {
    Actions,
    Levels,
    SharedRoles
}

pub struct Setting {
    pub guild_id: Id<GuildMarker>,
    pub enabled_modules: Vec<Module>,
    pub member_role_ids: Vec<Id<UserMarker>>,
    pub message_levels_enabled: bool,
    pub voice_levels_enabled: bool,
    pub rank_color: u32,
    pub should_keep_roles: bool
}

impl Module {
    fn as_str(&self) -> &'static str {
        match self {
            Module::Actions => "actions",
            Module::Levels => "levels",
            Module::SharedRoles => "shared_roles"
        }
    }
}

impl From<Row> for Setting {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            enabled_modules: row.get::<_, Vec<String>>(1).into_iter().map(|module| {
                match module.as_str() {
                    "actions" => Module::Actions,
                    "levels" => Module::Levels,
                    _ => Module::SharedRoles
                }                
            }).collect(),
            member_role_ids: row.get::<_, Vec<i64>>(2).into_iter().map(|id| Id::new(id as u64)).collect(),
            message_levels_enabled: row.get(3),
            voice_levels_enabled: row.get(4),
            rank_color: row.get::<_, i32>(5) as u32,
            should_keep_roles: row.get(6)
        }
    }
}

impl Database {
    pub async fn create_setting(&self, guild_id: Id<GuildMarker>) {
        let client = self.get_object().await;
        let query = "INSERT INTO setting(guild_id) VALUES($1) ON CONFLICT DO NOTHING;";

        client.query(query, &[&(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn delete_setting(&self, guild_id: Id<GuildMarker>) {
        let client = self.get_object().await;
        let query = "DELETE FROM setting WHERE guild_id = $1;";

        client.query(query, &[&(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn read_setting(&self, guild_id: Id<GuildMarker>) -> Option<Setting> {
        let client = self.get_object().await;
        let query = "SELECT * FROM setting WHERE guild_id = $1;";

        match client.query_one(query, &[&(guild_id.get() as i64)]).await {
            Ok(row) => Some(row.into()),
            Err(_) => None
        }
    }

    pub async fn update_enabled_modules(&self, guild_id: Id<GuildMarker>, enabled_modules: Vec<String>) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET enabled_modules = $2 WHERE guild_id = $1;";

        client.query(
            query,
            &[
                &enabled_modules.into_iter().map(|module| module).collect::<Vec<String>>(),
                &(guild_id.get() as i64)
            ]
        ).await.unwrap();
    }

    pub async fn update_message_levels_enabled(&self, guild_id: Id<GuildMarker>, state: bool) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET message_levels_enabled = $1 WHERE guild_id = $2;";

        client.query(query, &[&state, &(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn update_member_role_ids(&self, guild_id: Id<GuildMarker>, member_role_ids: Vec<Id<UserMarker>>) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET member_role_ids = $2 WHERE guild_id = $1;";

        client.query(
            query,
            &[
                &member_role_ids.into_iter().map(|id| id.get() as i64).collect::<Vec<i64>>(),
                &(guild_id.get() as i64)
            ]
        ).await.unwrap();
    }

    pub async fn update_rank_color(&self, guild_id: Id<GuildMarker>, color: u32) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET rank_color = $1 WHERE guild_id = $2;";
        
        client.query(query, &[&(color as i32), &(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn update_should_keep_roles(&self, guild_id: Id<GuildMarker>, state: bool) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET should_keep_roles = $1 WHERE guild_id = $2;";

        client.query(query, &[&state, &(guild_id.get() as i64)]).await.unwrap();
    }

    pub async fn update_voice_levels_enabled(&self, guild_id: Id<GuildMarker>, state: bool) {
        let client = self.get_object().await;
        let query = "UPDATE setting SET voice_levels_enabled = $1 WHERE guild_id = $2;";

        client.query(query, &[&state, &(guild_id.get() as i64)]).await.unwrap();
    }
}
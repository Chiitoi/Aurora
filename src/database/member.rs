use chrono::NaiveDateTime;
use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, UserMarker}};

pub struct Member {
    pub guild_id: Id<GuildMarker>,
    pub member_id: Id<UserMarker>,
    pub message_xp: u64,
    pub message_xp_updated_at: NaiveDateTime,
    pub voice_xp: u64,
    pub bio: String
}

impl From<Row> for Member {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            member_id: Id::new(row.get::<_, i64>(1) as u64),
            message_xp: row.get::<_, i64>(2) as u64,
            message_xp_updated_at: row.get::<_, NaiveDateTime>(3),
            voice_xp: row.get::<_, i64>(4) as u64,
            bio: row.get(5)
        }
    }
}

impl Database {
    pub async fn create_member(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>) {
        let client = self.get_object().await;
        let query = "INSERT INTO member(guild_id, member_id) VALUES($1, $2) ON CONFLICT DO NOTHING;";

        client.query(query, &[&(guild_id.get() as i64), &(member_id.get() as i64)]).await.unwrap();
    }

    pub async fn read_bio(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>) -> Option<String> {
        let client = self.get_object().await;
        let query = "SELECT bio FROM member WHERE guild_id = $1 AND member_id = $2;";

        match client.query_one(query, &[&(guild_id.get() as i64), &(member_id.get() as i64)]).await {
            Ok(row) => Some(row.get(0)),
            Err(_) => None
        }
    }

    pub async fn read_members(&self, guild_id: Id<GuildMarker>) -> Option<Vec<Member>> {
        let client = self.get_object().await;
        let query = "SELECT * FROM member WHERE guild_id = $1;";

        match client.query(query, &[&(guild_id.get() as i64)]).await {
            Ok(rows) => {
                let mut members = Vec::new();

                for row in rows.into_iter() {
                    members.push(Member::from(row));
                }

                Some(members)
            },
            Err(_) => None
        }
    }

    pub async fn read_xp(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>) -> Option<(u64, NaiveDateTime, u64)> {
        let client = self.get_object().await;
        let query = "SELECT message_xp, message_xp_updated_at, voice_xp FROM member WHERE guild_id = $1 AND member_id = $2;";

        match client.query_one(query, &[&(guild_id.get() as i64), &(member_id.get() as i64)]).await {
            Ok(row) => Some((
                row.get::<_, i64>(0) as u64,
                row.get::<_, NaiveDateTime>(1),
                row.get::<_, i64>(2) as u64,            
            )),
            Err(_) => None
        }
    }

    pub async fn update_bio(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>, bio: String) {
        let client = self.get_object().await;
        let query = "UPDATE member SET bio = $3 WHERE guild_id = $1 AND member_id = $2;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(member_id.get() as i64),
                &bio
            ]
        ).await.unwrap();
    }

    pub async fn update_message_xp(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>, xp: u64) {
        let client = self.get_object().await;
        let query = "UPDATE member SET message_xp = $3, message_xp_updated_at = CURRENT_TIMESTAMP WHERE guild_id = $1 AND member_id = $2;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(member_id.get() as i64),
                &(xp as i64)
            ]
        ).await.unwrap();
    }

    pub async fn update_voice_xp(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>, xp: u64) {
        let client = self.get_object().await;
        let query = "UPDATE member SET voice_xp = $3 WHERE guild_id = $1 AND member_id = $2;";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(member_id.get() as i64),
                &(xp as i64)
            ]
        ).await.unwrap();
    }


}
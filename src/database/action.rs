use crate::database::Database;
use tokio_postgres::Row;
use twilight_model::{id::{Id, marker::{GuildMarker, RoleMarker, UserMarker}}, guild::Guild};

pub struct Action {
    pub guild_id: Id<GuildMarker>,
    pub member_id: Id<UserMarker>,
    pub recipient_id: Id<UserMarker>,
    pub bite: u16,
    pub handhold: u16,
    pub hug: u16,
    pub kiss: u16,
    pub pat: u16,
    pub pinch: u16,
    pub poke: u16,
    pub punch: u16,
    pub tickle: u16,
}

impl From<Row> for Action {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>(0) as u64),
            member_id: Id::new(row.get::<_, i64>(1) as u64),
            recipient_id: Id::new(row.get::<_, i64>(2) as u64),
            bite: row.get::<_, i16>(3) as u16,
            handhold: row.get::<_, i16>(4) as u16,
            hug: row.get::<_, i16>(5) as u16,
            kiss: row.get::<_, i16>(6) as u16,
            pat: row.get::<_, i16>(7) as u16,
            pinch: row.get::<_, i16>(8) as u16,
            poke: row.get::<_, i16>(9) as u16,
            punch: row.get::<_, i16>(10) as u16,
            tickle: row.get::<_, i16>(11) as u16,
        }
    }
}

impl Database {
    pub async fn upsert_action(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>, recipient_id: Id<UserMarker>, action: String) {
        let client = self.get_object().await;
        let query = "
            INSERT INTO action(guild_id, member_id, recipient_id)
            VALUES($1, $2, $3)
            ON CONFLICT (guild_id, member_id, recipient_id)
            DO UPDATE SET $4 = EXCLUDED.$4 + 1;
        ";

        client.query(
            query,
            &[
                &(guild_id.get() as i64),
                &(member_id.get() as i64),
                &(recipient_id.get() as i64),
                &action
            ]
        ).await.unwrap();
    }
}
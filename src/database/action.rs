use crate::database::Database;
use std::fmt;
use tokio_postgres::Row;
use twilight_model::id::{Id, marker::{GuildMarker, UserMarker}};

pub struct CountedAction {
    pub cuddle: u16,
    pub handhold: u16,
    pub hug: u16,
    pub kiss: u16,
}

impl fmt::Display for CountedAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cuddle_text = if self.cuddle == 1 { ":heart: 1 cuddle".to_string() } else { format!(":heart: {} cuddles", self.cuddle) };
        let handhold_text = if self.handhold == 1 { ":handshake: 1 handhold".to_string() } else { format!(":handshake: {} handholds", self.cuddle) };
        let hug_text = if self.hug == 1 { ":hugging: 1 hug".to_string() } else { format!(":hugging: {} hugs", self.cuddle) };
        let kiss_text = if self.kiss == 1 { ":kissing_heart: 1 kiss".to_string() } else { format!(":kissing_heart: {} kisses", self.cuddle) };
        let counts = if self.cuddle + self.handhold + self.hug + self.kiss > 0 {
            let parts = vec![(self.cuddle, cuddle_text), (self.handhold, handhold_text), (self.hug, hug_text), (self.kiss, kiss_text)];
            
            parts.iter().filter_map(|(value, text)| {
                if *value > 0 { Some(format!("{text}")) } else { None }
            }).collect::<Vec<String>>().join("\n")
        } else { ":pensive: No counted actions...".to_string() };

        write!(f, "{counts}")
    }
}

impl From<Row> for CountedAction {
    fn from(row: Row) -> Self {
        Self {
            cuddle: row.get::<_, i16>(0) as u16,
            handhold: row.get::<_, i16>(1) as u16,
            hug: row.get::<_, i16>(2) as u16,
            kiss: row.get::<_, i16>(3) as u16,
        }
    }
}



impl Database {
    pub async fn read_action_counts(&self, guild_id: Id<GuildMarker>, id_one: Id<UserMarker>, id_two: Id<UserMarker>) -> CountedAction {
        let client = self.get_object().await;
        let query = "
            SELECT
                COUNT(cuddle)::INT2 AS cuddle,
                COUNT(handhold)::INT2 AS handhold,
                COUNT(hug)::INT2 AS hug,
                COUNT(kiss)::INT2 AS kiss
            FROM
                action
            WHERE
                guild_id = $1
                AND ((member_id = $2 AND recipient_id = $3) OR (member_id = $3 AND recipient_id = $2));
        ";

        client.query_one(
            query,
            &[
                &(guild_id.get() as i64),
                &(id_one.get() as i64),
                &(id_two.get() as i64)
            ]
        ).await.unwrap().into()
    }

    pub async fn read_kill_counts(&self, guild_id: Id<GuildMarker>, id_one: Id<UserMarker>, id_two: Id<UserMarker>) -> (u16, u16) {
        let client = self.get_object().await;
        let query = "
            WITH id_one AS (
                SELECT
                    COALESCE(SUM(kill), 0)::INT2 AS id_one
                FROM
                    action
                WHERE
                    guild_id = $1
                    AND (member_id = $2 AND recipient_id = $3)
            ),
            id_two AS (
                SELECT
                    COALESCE(SUM(kill), 0)::INT2 AS id_two
                FROM
                    action
                WHERE
                    guild_id = $1
                    AND (member_id = $3 AND recipient_id = $2)
            )
            SELECT
                *
            FROM
                id_one,
                id_two;
        ";
        let row = client.query_one(
            query,
            &[
                &(guild_id.get() as i64),
                &(id_one.get() as i64),
                &(id_two.get() as i64)
            ]
        ).await.unwrap();
        
        (row.get::<_, i16>(0) as u16, row.get::<_, i16>(1) as u16)
    }

    pub async fn upsert_action(&self, guild_id: Id<GuildMarker>, member_id: Id<UserMarker>, recipient_id: Id<UserMarker>, action: &str) -> u16 {
        let client = self.get_object().await;
        let query = format!("
            INSERT INTO action(guild_id, member_id, recipient_id, {action})
            VALUES($1, $2, $3, 1)
            ON CONFLICT (guild_id, member_id, recipient_id)
            DO UPDATE SET {action} = action.{action} + 1
            RETURNING {action};
        ");

        match client.query_one(
            &query,
            &[
                &(guild_id.get() as i64),
                &(member_id.get() as i64),
                &(recipient_id.get() as i64)
            ]
        ).await {
            Ok(row) => row.get::<_, i16>(0) as u16,
            Err(_) => 1u16,
        }
    }
}
pub mod action;
pub mod level_role;
pub mod shared_role;
pub mod setting;
pub mod member;

use crate::constants::DATABASE_URL;
use deadpool_postgres::{Client, Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
use tokio_postgres::{Config, NoTls};

pub struct Database {
    pool: Pool
}

impl Database {
    pub fn new() -> Self {
        let pool = Pool::builder(Manager::from_config(
            Config::from_str(&DATABASE_URL).unwrap(),
            NoTls,
            ManagerConfig { recycling_method: RecyclingMethod::Fast }
        ))
            .max_size(16)
            .build()
            .unwrap();

        Self {
            pool
        }
    }

    async fn get_object(&self) -> Client {
        self.pool.get().await.unwrap()
    }

    pub async fn create_tables(&self) {
        let client = self.get_object().await;
        let schema_query = "
            DROP TYPE IF EXISTS module CASCADE;
            DROP TYPE IF EXISTS role_kind CASCADE;

            CREATE TYPE module AS ENUM ('actions', 'levels', 'shared_roles');
            CREATE TYPE role_kind AS ENUM ('message', 'voice');

            CREATE TABLE IF NOT EXISTS public.action (
                guild_id INT8 NOT NULL,
                member_id INT8 NOT NULL,
                recipient_id INT8 NOT NULL,                
                bite INT2 NOT NULL DEFAULT 0,
                cuddle INT2 NOT NULL DEFAULT 0,
                handhold INT2 NOT NULL DEFAULT 0,
                hug INT2 NOT NULL DEFAULT 0,
                kill INT2 NOT NULL DEFAULT 0,
                kiss INT2 NOT NULL DEFAULT 0,
                pat INT2 NOT NULL DEFAULT 0,
                pinch INT2 NOT NULL DEFAULT 0,
                poke INT2 NOT NULL DEFAULT 0,
                punch INT2 NOT NULL DEFAULT 0,
                tickle INT2 NOT NULL DEFAULT 0,
                CONSTRAINT ck_action PRIMARY KEY (guild_id, member_id, recipient_id)
            );
            CREATE TABLE IF NOT EXISTS public.level_role (
                guild_id INT8 NOT NULL,
                role_id INT8 NOT NULL,
                kind role_kind NOT NULL,
                level INT2 NOT NULL,
                is_persistent BOOLEAN NOT NULL DEFAULT FALSE,
                CONSTRAINT ck_level_role PRIMARY KEY (guild_id, role_id, kind, level)
            );
            CREATE TABLE IF NOT EXISTS public.member (
                guild_id INT8 NOT NULL,
                member_id INT8 NOT NULL,
                message_xp INT8 NOT NULL DEFAULT 0,
                message_xp_updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
                voice_xp INT8 NOT NULL DEFAULT 0,
                bio TEXT DEFAULT NULL,
                spouse_id INT8 DEFAULT NULL,
                CONSTRAINT ck_member PRIMARY KEY (guild_id, member_id)
            );
            CREATE TABLE IF NOT EXISTS public.setting (
                guild_id INT8 NOT NULL,
                enabled_modules module[] NOT NULL DEFAULT '{}',                
                member_role_ids INT8[] NOT NULL DEFAULT '{}',
                message_levels_enabled BOOLEAN NOT NULL DEFAULT FALSE,
                voice_levels_enabled BOOLEAN NOT NULL DEFAULT FALSE,
                rank_color INT4 NOT NULL DEFAULT 16758725,
                should_keep_roles BOOLEAN NOT NULL DEFAULT FALSE,
                CONSTRAINT pk_setting PRIMARY KEY (guild_id)
            );
            CREATE TABLE IF NOT EXISTS public.shared_role (
                guild_id INT8 NOT NULL,
                role_id INT8 NOT NULL,
                owner_ids INT8[] NOT NULL DEFAULT '{}',
                CONSTRAINT ck_shared_role PRIMARY KEY (guild_id, role_id)
            );
        ";

        client.batch_execute(schema_query).await.unwrap();
    }
}
use crate::DynoConfig;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Default, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTest {
    pub id: i64,
    pub user_id: i64,
    pub info_id: Option<i64>,
    pub uuid: Uuid,
    pub data_url: String,
    pub data_checksum: String,
    pub verified: bool,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTestDataInfo {
    pub checksum_hex: String,
    pub config: DynoConfig,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}

use crate::infomotor::InfoMotor;
use chrono::{DateTime, Utc};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTest {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub motor: InfoMotor,
    pub data_url: String,
    pub data_url_excel: String,
    pub data_url_csv: String,
    pub data_url_pdf: String,
    pub verified: bool,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTestHttp {
    pub data_size: usize,
    pub motor: InfoMotor,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
}

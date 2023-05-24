use crate::DynoConfig;
use chrono::NaiveDateTime;

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTest {
    pub id: i64,
    pub user_id: i64,
    pub info_id: i64,
    pub data_url: String,
    pub data_url_excel: String,
    pub data_url_csv: String,
    pub data_url_pdf: String,
    pub verified: bool,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[repr(u32)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "backend", sqlx(type_name = "motor_type"))]
pub enum MotorTy {
    Electric = 0,
    #[default]
    Engine = 1,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct MotorInfo {
    pub name: Option<String>,
    pub cc: Option<u32>,
    pub cylinder: Option<u32>,
    pub stroke: Option<u32>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTestInfo {
    pub id: i64,
    pub motor_type: MotorTy,

    #[cfg_attr(feature = "backend", sqlx(flatten))]
    pub motor_info: MotorInfo,

    pub diameter_roller: Option<f64>,
    pub diameter_roller_beban: Option<f64>,
    pub diameter_gear_encoder: Option<f64>,
    pub diameter_gear_beban: Option<f64>,
    pub jarak_gear: Option<f64>,
    pub berat_beban: Option<f64>,
    pub gaya_beban: Option<f64>,
    pub keliling_roller: Option<f64>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTestDataInfo {
    pub data_size: usize,
    pub config: DynoConfig,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}

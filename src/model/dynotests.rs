use crate::DynoConfig;
use chrono::NaiveDateTime;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTest {
    pub id: u32,
    pub user_id: u32,
    pub info_id: u32,
    pub data_url: String,
    pub data_checksum: String,
    pub verified: bool,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[repr(u32)]
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
pub enum MotorTy {
    Electric = 0,
    #[default]
    Engine = 1,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct MotorInfo {
    pub name: Option<String>,
    pub cc: Option<u32>,
    pub cylinder: Option<u32>,
    pub stroke: Option<u32>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoTestInfo {
    pub id: u32,
    pub motor_type: MotorTy,
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
    pub checksum_hex: String,
    pub config: DynoConfig,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}

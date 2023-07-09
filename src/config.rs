use crate::{
    convertions::{length, weight},
    data_structure::filter::DataFilter,
    MotorInfo, MotorType, Numeric,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DynoConfig {
    #[serde(default = "default_mqtt_user")]
    pub mqtt_user: String,
    #[serde(default = "default_mqtt_pswd")]
    pub mqtt_pswd: String,

    #[serde(default)]
    pub motor_info: MotorInfo,
    #[serde(default)]
    pub motor_type: MotorType,

    #[serde(default = "default_diameter_roller")]
    pub diameter_roller: length::Metres,

    #[serde(default = "default_diameter_roller_beban")]
    pub diameter_roller_beban: length::Metres,

    #[serde(default = "default_diameter_gear_encoder")]
    pub diameter_gear_encoder: length::Metres,

    #[serde(default = "default_diameter_gear_beban")]
    pub diameter_gear_beban: length::Metres,

    #[serde(default = "default_jarak_gear")]
    pub jarak_gear: length::Metres,

    #[serde(default = "default_berat_beban")]
    pub berat_beban: weight::KiloGram,

    #[serde(default = "default_gaya_beban")]
    pub gaya_beban: crate::Float,

    #[serde(default = "default_keliling_roller")]
    pub keliling_roller: length::Metres,

    #[serde(default)]
    pub filter: DataFilter,
}

impl Default for DynoConfig {
    fn default() -> Self {
        Self {
            mqtt_user: default_mqtt_user(),
            mqtt_pswd: default_mqtt_pswd(),
            diameter_roller: default_diameter_roller(),
            diameter_roller_beban: default_diameter_roller_beban(),
            berat_beban: default_berat_beban(),
            diameter_gear_encoder: default_diameter_gear_encoder(),
            diameter_gear_beban: default_diameter_gear_beban(),
            jarak_gear: default_jarak_gear(),
            gaya_beban: default_gaya_beban(),
            keliling_roller: default_keliling_roller(),
            motor_type: MotorType::default(),
            motor_info: MotorInfo::default(),
            filter: DataFilter::default(),
        }
    }
}

impl DynoConfig {
    pub fn init(&mut self) {
        self.filter.reset();
        self.gaya_beban = self.berat_beban.value() * crate::GRAVITY_SPEED;
        self.keliling_roller = self.diameter_roller * crate::PI;
    }
    #[inline(always)]
    pub fn perbandingan_gear(&self) -> crate::Float {
        (self.diameter_gear_beban / self.diameter_roller).to_float()
    }

    #[inline(always)]
    pub fn inertia_roller_beban(&self) -> crate::Float {
        0.5 * self.berat_beban.to_float() * self.diameter_roller_beban.to_float().powi(2)
    }
}
#[inline(always)]
fn default_diameter_roller() -> length::Metres {
    length::Metres(0.1422) // 14.22 cm
}
#[inline(always)]
fn default_diameter_roller_beban() -> length::Metres {
    length::Metres(0.1933) //  19.33 cm
}
#[inline(always)]
fn default_diameter_gear_encoder() -> length::Metres {
    length::Metres(0.1)
}
#[inline(always)]
fn default_diameter_gear_beban() -> length::Metres {
    length::Metres(0.054)
}
#[inline(always)]
fn default_jarak_gear() -> length::Metres {
    length::Metres(0.144)
}
#[inline(always)]
fn default_berat_beban() -> weight::KiloGram {
    weight::KiloGram(18.5)
}
#[inline(always)]
fn default_gaya_beban() -> crate::Float {
    default_berat_beban().value() * crate::GRAVITY_SPEED
}
#[inline(always)]
fn default_keliling_roller() -> length::Metres {
    default_diameter_roller() * crate::PI
}
#[inline(always)]
fn default_mqtt_user() -> String {
    "E32201406_RIZAL".to_owned()
}
#[inline(always)]
fn default_mqtt_pswd() -> String {
    "E32201406_RIZAL".to_owned()
}

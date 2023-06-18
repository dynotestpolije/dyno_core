use crate::{
    convertions::{length, weight},
    data_structure::ExponentialFilter,
    MotorType, Numeric, RotationPerMinute,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DynoConfig {
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

    pub filter_rpm_engine: ExponentialFilter<RotationPerMinute>,
}

impl Default for DynoConfig {
    fn default() -> Self {
        Self {
            diameter_roller: default_diameter_roller(),
            diameter_roller_beban: default_diameter_roller_beban(),
            berat_beban: default_berat_beban(),
            diameter_gear_encoder: default_diameter_gear_encoder(),
            diameter_gear_beban: default_diameter_gear_beban(),
            jarak_gear: default_jarak_gear(),
            gaya_beban: default_gaya_beban(),
            keliling_roller: default_keliling_roller(),
            motor_type: MotorType::default(),
            filter_rpm_engine: ExponentialFilter::new(16),
        }
    }
}

impl DynoConfig {
    pub fn load_from_config<P: AsRef<std::path::Path>>(path: P) -> Self {
        let content = match std::fs::read_to_string(path.as_ref()) {
            Ok(k) => k,
            Err(err) => {
                log::error!("{err}");
                return Default::default();
            }
        };
        match path
            .as_ref()
            .extension()
            .map(|x| x.to_str().unwrap_or_default())
        {
            Some("toml") => toml::from_str(content.as_str())
                .map_err(|err| log::error!("{err}"))
                .unwrap_or_default(),
            Some("json") => serde_json::from_slice(content.as_bytes())
                .map_err(|err| log::error!("{err}"))
                .unwrap_or_default(),
            _ => Default::default(),
        }
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

#[repr(u8)]
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Stroke {
    Two = 2,

    #[default]
    Four = 4,
}

impl Stroke {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [Self::Two, Self::Four].into_iter()
    }
}

impl std::fmt::Display for Stroke {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Stroke::Two => write!(f, "Two Stroke"),
            Stroke::Four => write!(f, "Four Stroke"),
        }
    }
}

impl From<u8> for Stroke {
    #[inline(always)]
    fn from(val: u8) -> Self {
        match val {
            2 => Self::Two,
            4 => Self::Four,
            _ => Self::default(),
        }
    }
}

#[repr(u8)]
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Transmition {
    Unknown,
    #[default]
    Automatic,
    Manual,
}
impl Transmition {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [Self::Unknown, Self::Automatic, Self::Manual].into_iter()
    }
}

impl std::fmt::Display for Transmition {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Transmition::Unknown => write!(f, "Unknown Transmition"),
            Transmition::Manual => write!(f, "Manual Transmition"),
            Transmition::Automatic => write!(f, "Automatic Transmition"),
        }
    }
}
impl From<u8> for Transmition {
    #[inline(always)]
    fn from(val: u8) -> Self {
        match val {
            0 => Transmition::Automatic,
            1..=8 => Transmition::Manual,
            _ => Transmition::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Cylinder {
    #[default]
    Single = 1,
    Two,
    Triple,
    Four,
    Six,
    Eight,
}

impl Cylinder {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [
            Self::Single,
            Self::Two,
            Self::Triple,
            Self::Four,
            Self::Six,
            Self::Eight,
        ]
        .into_iter()
    }
}

impl std::fmt::Display for Cylinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Cylinder::Single => write!(f, "Single Cylinder"),
            Cylinder::Two => write!(f, "Two Cylinder"),
            Cylinder::Triple => write!(f, "Three Cylinder"),
            Cylinder::Four => write!(f, "Four Cylinder"),
            Cylinder::Six => write!(f, "Six Cylinder"),
            Cylinder::Eight => write!(f, "Eight Cylinder"),
        }
    }
}
impl From<u8> for Cylinder {
    fn from(val: u8) -> Self {
        match val {
            1 => Cylinder::Single,
            2 => Cylinder::Two,
            3 => Cylinder::Triple,
            4 => Cylinder::Four,
            6 => Cylinder::Six,
            8 => Cylinder::Eight,
            _ => Cylinder::default(),
        }
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum MotorType {
    #[default]
    Engine,
    Electric,
}

impl MotorType {
    pub fn is_electric(&self) -> bool {
        matches!(self, Self::Electric)
    }

    pub fn is_engine(&self) -> bool {
        matches!(self, Self::Engine)
    }
}

impl From<u8> for MotorType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Engine,
            1 => Self::Electric,
            _ => Self::Engine,
        }
    }
}

#[derive(Debug, Clone, PartialEq, derive_more::Display, serde::Deserialize, serde::Serialize)]
#[display(fmt = "[{name} - {cc}|{cylinder}|{stroke}]")]
#[serde(default)]
pub struct MotorInfo {
    pub name: String,
    pub cc: u32,
    pub cylinder: Cylinder,
    pub stroke: Stroke,
    pub transmition: Transmition,
}

impl Default for MotorInfo {
    fn default() -> Self {
        Self {
            name: "Default Info".to_owned(),
            cc: 125,
            cylinder: Cylinder::Single,
            stroke: Stroke::Four,
            transmition: Transmition::Manual,
        }
    }
}

impl MotorInfo {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: impl ToString) -> &mut Self {
        self.name = name.to_string();
        self
    }

    #[inline(always)]
    pub fn set_cc(&mut self, cc: impl Into<u32>) -> &mut Self {
        self.cc = cc.into();
        self
    }

    #[inline(always)]
    pub fn set_cylinder(&mut self, cylinder: impl Into<Cylinder>) -> &mut Self {
        self.cylinder = cylinder.into();
        self
    }

    #[inline(always)]
    pub fn set_stroke(&mut self, stroke: impl Into<Stroke>) -> &mut Self {
        self.stroke = stroke.into();
        self
    }

    #[inline(always)]
    pub fn set_transmition(&mut self, transmition: impl Into<Transmition>) -> &mut Self {
        self.transmition = transmition.into();
        self
    }
}

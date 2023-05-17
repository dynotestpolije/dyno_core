pub trait Angular: crate::Numeric {
    #[inline]
    fn to_radians_per_second(self) -> RadiansPerSecond {
        RadiansPerSecond::new(self)
    }

    #[inline]
    fn to_rotation_per_minute(self) -> RotationPerMinute {
        RotationPerMinute::new(self)
    }
}

super::declare_convertion_type!(Angular => self {
    RotationPerMinute["rpm"] [ to_radians_per_second => RadiansPerSecond  { self.0 * 0.104_72 } ],
    RadiansPerSecond["rps"] [ to_rotation_per_minute => RotationPerMinute { self.0 * 9.549_297 } ]
});

impl RotationPerMinute {
    pub fn from_rot(rot: crate::Float, ms: crate::Float) -> Self {
        Self(super::prelude::EncoderTicks::per_minute(rot, ms))
    }
}

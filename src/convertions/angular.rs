pub trait Angular: crate::Numeric {
    /// Converts Self to RadiansPerSecond
    #[inline]
    fn to_radians_per_second(self) -> RadiansPerSecond {
        RadiansPerSecond::new(self)
    }
    /// Converts Self to RotationPerMitute
    #[inline]
    fn to_rotation_per_minute(self) -> RotationPerMinute {
        RotationPerMinute::new(self)
    }
}

super::declare_convertion_type!(Angular => self {

    RotationPerMinute["rpm"] [ to_radians_per_second => RadiansPerSecond  { self.0 * 0.104_72 } ],

    RadiansPerSecond["rps"] [ to_rotation_per_minute => RotationPerMinute { self.0 * 9.549_297 } ]

});

pub trait Power: crate::Numeric {
    fn to_horse_power(self) -> HorsePower {
        HorsePower::new(self)
    }

    fn to_kilo_watt(self) -> KiloWatt {
        KiloWatt::new(self)
    }

    fn to_watt(self) -> Watt {
        Watt::new(self)
    }
}

super::declare_convertion_type!(Power => self {
    HorsePower["Hp"] [
        to_kilo_watt        => KiloWatt { self.0 * 0.7457   },
        to_watt             => Watt     { self.0 * 745.7    }
    ],
    KiloWatt["kW"] [
        to_horse_power  => HorsePower   { self.0 * 1.34102 },
        to_watt         => Watt         { self.0 * 1000.0 }
    ],
    Watt["W"] [
        to_horse_power  => HorsePower   { self.0 * 0.00134102 },
        to_kilo_watt    => KiloWatt     { self.0 * 0.001 }
    ]
});

impl HorsePower {
    #[inline]
    pub fn from_nm(
        torsi: super::torque::NewtonMeter,
        rpm: super::angular::RotationPerMinute,
    ) -> Self {
        Self(torsi.value() * rpm.value() / 9549.)
    }

    #[inline]
    pub fn from_lsb(
        torsi: super::torque::PoundFoot,
        rpm: super::angular::RotationPerMinute,
    ) -> Self {
        Self(torsi.value() * rpm.value() / 9549.)
    }
}

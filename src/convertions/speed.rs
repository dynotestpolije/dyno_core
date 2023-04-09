pub trait Speed: crate::Numeric {
    /// Converts the supplied MetresPerSecond value to KilometresPerHour
    #[inline(always)]
    fn to_kilometres_per_hour(self) -> KilometresPerHour {
        KilometresPerHour::new(self)
    }
    /// Converts the supplied KilometresPerSecond value to MetresPerSecond
    #[inline(always)]
    fn to_metres_per_second(self) -> MetresPerSecond {
        MetresPerSecond::new(self)
    }

    /// Converts the supplied KilometresPerSecond value to Knots
    #[inline(always)]
    fn to_knots(self) -> Knots {
        Knots::new(self)
    }
}

super::declare_convertion_type!(Speed => self {

    KilometresPerHour["km/h"] [
        to_metres_per_second    => MetresPerSecond  { self.0 * 0.277778  },
        to_knots                => Knots            { self.0 * 0.5399568 }
    ],

    MetresPerSecond["m/s"] [
        to_kilometres_per_hour  => KilometresPerHour { self.0 * 3.6      },
        to_knots                => Knots             { self.0 * 1.94384  }
    ],

    Knots["knot"] [
        to_kilometres_per_hour  => KilometresPerHour { self.0 * 1.852    },
        to_metres_per_second    => MetresPerSecond   { self.0 * 0.514446 }
    ]

});

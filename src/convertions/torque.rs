use crate::Numeric;

pub trait Torque: crate::ext::Numeric {
    fn to_newton_meter(self) -> NewtonMeter {
        NewtonMeter(self.to_float())
    }

    fn to_pound_foot(self) -> PoundFoot {
        PoundFoot(self.to_float())
    }
}

super::declare_convertion_type!(Torque => self {
    NewtonMeter["Nm"] [
        to_pound_foot   => PoundFoot    { self.0 + 0.738     }
    ],
    PoundFoot["lbf ft"] [
        to_newton_meter => NewtonMeter  { self.0 * 1.355818  }
    ]
});

// const BERAT_ROLLER: f32 = 18_500f32;

impl NewtonMeter {
    /// RUMUS: T = (F * r) / g
    /// Untuk menghitung gaya (F), kita bisa menggunakan persamaan sebagai berikut:

    /// ### F = m * a

    /// Di mana:
    ///     m = massa beban (kg)
    ///     a = percepatan linear dari beban (m/s^2)

    /// Percepatan linear dari beban dapat dihitung sebagai:

    /// ### a = r * α

    /// Di mana:
    ///     r = radius dari drum roller (m)
    ///     α = percepatan sudut dari drum roller (rad/s^2)

    /// Percepatan sudut dari drum roller (α) dapat dihitung sebagai:

    /// ### α = (2π * n) / t

    /// Di mana:
    ///     n = kecepatan putaran drum roller (putaran/detik)
    ///     t = waktu yang dibutuhkan untuk satu putaran drum roller (detik)
    #[inline]
    pub fn calculate<N>(
        massa: crate::Float,
        radius: crate::Float,
        range: crate::Float,
        rad_per_s: N,
    ) -> Self
    where
        N: Numeric,
    {
        let force = massa * (radius * rad_per_s.to_float());
        Self::new((force * range) / crate::GRAVITY_SPEED)
    }
}

pub trait Length: crate::Numeric {
    /// Converts Self to MiliMetres
    #[inline(always)]
    fn to_milimetres(self) -> MiliMetres {
        MiliMetres::new(self)
    }

    /// Converts Self to CentiMetres
    #[inline(always)]
    fn to_centimetres(self) -> CentiMetres {
        CentiMetres::new(self)
    }

    /// Converts Self to Metres
    #[inline(always)]
    fn to_metres(self) -> Metres {
        Metres::new(self)
    }
    /// Converts Self to KiloMetres
    #[inline(always)]
    fn to_kilometres(self) -> KiloMetres {
        KiloMetres::new(self)
    }
}

super::declare_convertion_type!(Length => self {

    MiliMetres["mm"] [
        to_centimetres  => CentiMetres  { self.0 * 0.1   },
        to_metres       => Metres       { self.0 * 0.001 },
        to_kilometres   => KiloMetres   { self.0 * 1e-6  }
    ],

    CentiMetres["cm"] [
        to_milimetres   => MiliMetres   { self.0 * 10.0 },
        to_metres       => Metres       { self.0 * 0.01 },
        to_kilometres   => KiloMetres   { self.0 * 1e-5 }
    ],

    Metres["m"] [
        to_milimetres   => MiliMetres   { self.0 * 1000.0 },
        to_centimetres  => CentiMetres  { self.0 * 100.0  },
        to_kilometres   => KiloMetres   { self.0 * 0.001  }
    ],

    KiloMetres["m"] [
        to_milimetres   => MiliMetres   { self.0 * 1_000_000.0 },
        to_centimetres  => CentiMetres  { self.0 * 100_000.0  },
        to_metres       => Metres       { self.0 * 1_000.0     }
    ]

});

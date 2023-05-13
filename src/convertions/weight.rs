pub trait Weight: crate::ext::Numeric {
    fn to_kilogram(self) -> KiloGram {
        KiloGram::new(self)
    }

    fn to_gram(self) -> Gram {
        Gram::new(self)
    }

    fn to_pound(self) -> Pound {
        Pound::new(self)
    }
}

super::declare_convertion_type!(Weight => self {
    KiloGram["Kg"] [
        to_gram     => Gram     { self.0 * 1000.  },
        to_pound    => Pound    { self.0 * 2.205  }
    ],
    Gram["g"] [
        to_kilogram => KiloGram { self.0 * 0.001        },
        to_pound    => Pound    { self.0 * 0.00220462   }
    ],
    Pound["lb"] [
        to_kilogram => KiloGram { self.0 * 0.453592 },
        to_gram     => Gram     { self.0 * 453.6    }
    ]
});

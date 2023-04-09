pub trait Temperature: crate::ext::Numeric {
    fn to_celcius(self) -> Celcius {
        Celcius(self.to_float())
    }
    fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.to_float())
    }
    fn to_kelvin(self) -> Kelvin {
        Kelvin(self.to_float())
    }
}

super::declare_convertion_type!(Temperature => self {

    Celcius["°C"] [
        to_fahrenheit   => Fahrenheit   { (self.0 * 1.8) + 32.0   },
        to_kelvin       => Kelvin       { self.0 + 273.15         }
    ],

    Fahrenheit["°F"] [
        to_celcius      => Celcius      { (self.0 - 32.0) / 1.8             },
        to_kelvin       => Kelvin       { ((self.0 - 32.0) / 1.8) + 273.15  }
    ],

    Kelvin["K"] [
        to_fahrenheit   => Fahrenheit   { (self.0 - 273.15) * 1.8 + 32.0    },
        to_celcius      => Celcius      { self.0 - 273.15                   }
    ]

});

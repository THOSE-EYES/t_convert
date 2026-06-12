#![no_std]

/// Create enum 'Unit' holding the temperature units.
#[derive(Debug, PartialEq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Temperature struct holding the value of the input and the unit which has a type from the 'Unit' enum.
#[derive(Debug)]
pub struct Temperature {
    value: f64,
    unit: Unit,
}

/// An implementation of the temperature struct.
impl Temperature {
    pub const fn new(value:f64, unit: Unit) -> Option<Self> {
        let valid_temp = match unit {
            Unit::Celsius => value >= -273.15,
            Unit::Fahrenheit => value >= -459.67,
            Unit::Kelvin => value >= 0.0,
        };

        if valid_temp {
            Some(Self { value, unit })
        } else {
            None
        }
    }

    /// To get the value from a result variable holding the converted unit, use this method!
    pub const fn get_value(&self) -> f64 {
        self.value
    }

    /// A function to convert either celsius, fahrenheit or kelvin to celsius using a floating point input.
    ///
    /// Convert a temp to Celsius from Fahrenheit.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// // We use '.expect' to get the value since it's an 'Option' value.
    /// let temp = Temperature::new(0.0, Unit::Fahrenheit).expect("Invalid value.");
    ///
    /// // We match the value to the 'to_celsius' method to ensure safety.
    /// match temp.to_celsius() {
    ///     Some(t) => println!("Temperature in celsius: {}", t.get_value()),
    ///     None => println!("Couldn't convert temperature!"),
    /// };
    ///
    /// ```
    pub const fn to_celsius(&self) -> Option<Self> {
        let celsius = match self.unit {
            Unit::Celsius => self.value,

            Unit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,

            Unit::Kelvin => self.value - 273.15,
        };

        Self::new(celsius, Unit::Celsius)
    }

    /// A function to convert either fahrenheit, celsius or kelvin to fahrenheit using a floating point input.
    ///
    /// Convert a temp to Fahrenheit from celsius.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// // We use '.expect' to get the value since it's an 'Option' value.
    /// let temp = Temperature::new(0.0, Unit::Celsius).expect("Invalid value.");
    ///
    /// // We match the value to the 'to_fahrenheit' method to ensure safety.
    /// match temp.to_fahrenheit() {
    ///     Some(t) => println!("Temperature in fahrenheit: {}", t.get_value()),
    ///     None => println!("Couldn't convert temperature!"),
    /// };
    ///
    /// ```
    pub const fn to_fahrenheit(&self) -> Option<Self> {
        let fahrenheit = match self.unit {
            Unit::Fahrenheit => self.value,

            Unit::Celsius => (self.value * 9.0 / 5.0) + 32.0,

            Unit::Kelvin => (self.value - 273.15) * 9.0 / 5.0 + 32.0,
        };

        Self::new(fahrenheit, Unit::Fahrenheit)
    }

    /// A function to convert either kelvin, celsius or fahrenheit to kelvin using a floating point input.
    ///
    /// Convert a temp to Kelvin from celsius.
    /// ```
    ///
    /// use t_convert::{Temperature, Unit};
    ///
    /// // We use '.expect' to get the value since it's an 'Option' value.
    /// let temp = Temperature::new(0.0, Unit::Celsius).expect("Invalid value.");
    ///
    /// // We match the value to the 'to_kelvin' method to ensure safety.
    /// match temp.to_kelvin() {
    ///     Some(t) => println!("Temperature in kelvin: {}", t.get_value()),
    ///     None => println!("Couldn't convert temperature!"),
    /// };
    ///
    /// ```
    pub const fn to_kelvin(&self) -> Option<Self> {
        let kelvin = match self.unit {
            Unit::Kelvin => self.value,

            Unit::Celsius => self.value + 273.15,

            Unit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
        };

        Self::new(kelvin, Unit::Kelvin)
    }
}

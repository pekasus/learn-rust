// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

const SECONDS_PER_EARTH_YEAR: f64 = 31557600f64;

pub trait Planet {
    const FACTOR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / SECONDS_PER_EARTH_YEAR / Self::FACTOR
    }
}

macro_rules! planets {
    ($($planet_name:ident => $factor:literal;)+) => {
        $(
            pub struct $planet_name;
            impl Planet for $planet_name {
                const FACTOR: f64 = $factor;
            }
        )*
    };
}

planets!(
    Mercury => 0.2408467;
    Venus => 0.61519726;
    Earth => 1.0;
    Mars => 1.8808158;
    Jupiter => 11.862615;
    Saturn => 29.447498;
    Uranus => 84.016846;
    Neptune => 164.79132;
);

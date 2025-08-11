// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
// My favorite solution!

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / ((60.0 * 60.0 * 24.0 * 365.25)))
    }
}

pub trait Planet {
    const ORBIT_YEAR_RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBIT_YEAR_RATIO
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBIT_YEAR_RATIO: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBIT_YEAR_RATIO: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBIT_YEAR_RATIO: f64 = 1.0;
}
impl Planet for Mars {
    const ORBIT_YEAR_RATIO: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBIT_YEAR_RATIO: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBIT_YEAR_RATIO: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBIT_YEAR_RATIO: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBIT_YEAR_RATIO: f64 = 164.79132;
}

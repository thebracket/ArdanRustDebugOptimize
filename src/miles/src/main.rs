#[derive(Clone, Debug)]
enum LengthUnit {
    Meters,
    Millimeters,
    Miles,
}

impl LengthUnit {
    fn units_per_nanometer(&self) -> i64 {
        match self {
            LengthUnit::Meters => 1_000_000_000,
            LengthUnit::Millimeters => 1_000_000,
            LengthUnit::Miles => 1_609_344_000_000,
        }
    }
}

struct Length {
    nanometers: i64,
    unit: LengthUnit,
}

impl Length {
    fn from(unit: LengthUnit, value: i64) -> Self {
        Self {
            nanometers: value * unit.units_per_nanometer(),
            unit,
        }
    }

    fn to(&self, unit: LengthUnit) -> Self {
        Self {
            nanometers: self.nanometers,
            unit
        }
    }

    fn value(&self) -> f64 {
        self.nanometers as f64 / self.unit.units_per_nanometer() as f64
    }
}

fn walking(distance: Length) -> String {
    let miles = distance.to(LengthUnit::Miles);
    format!("{} miles", miles.value())
}

fn main() {
    let miles = Length::from(LengthUnit::Miles, 500);
    let meters = Length::from(LengthUnit::Meters, 500);
    println!(
        "If I would walk {}, then I would walk {} more",
        walking(miles),
        walking(meters),
    );
}
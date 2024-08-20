pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.min - self.max
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    const EMPTY: Self = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    const UNIVERSE: Self = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}

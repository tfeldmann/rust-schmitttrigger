pub struct SchmittTrigger<T> {
    lower: T,
    upper: T,
    state: bool,
}

impl<T: PartialOrd> SchmittTrigger<T> {
    pub fn new(lower: T, upper: T) -> SchmittTrigger<T> {
        SchmittTrigger {
            lower,
            upper,
            state: false,
        }
    }

    pub fn set_thresholds(&mut self, lower: T, upper: T) {
        self.lower = lower;
        self.upper = upper;
    }

    pub fn input(&mut self, value: T) -> bool {
        if value >= self.upper {
            self.state = true;
        }
        if value <= self.lower {
            self.state = false;
        }
        self.state
    }

    pub fn output(&self) -> bool {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        let mut trigger = SchmittTrigger::new(5, 10);
        assert_eq!(trigger.input(4), false);
        assert_eq!(trigger.input(5), false);
        assert_eq!(trigger.input(11), true);
        trigger.input(12);
        assert_eq!(trigger.input(8), true);
        assert_eq!(trigger.input(4), false);
    }

    #[test]
    fn test_floats() {
        let mut trigger = SchmittTrigger::new(0.3, 0.6);
        trigger.input(0.0);
        assert_eq!(trigger.output(), false);
        trigger.input(1.0);
        assert_eq!(trigger.output(), true);
        trigger.input(0.5);
        assert_eq!(trigger.output(), true);
        trigger.input(0.2);
        assert_eq!(trigger.output(), false);
        trigger.input(0.5);
        assert_eq!(trigger.output(), false);
    }
}

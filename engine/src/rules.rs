pub struct SimpleRules {
    pub survives: [bool; 7],
    pub emerges: [bool; 7],
}

impl SimpleRules {
    pub fn apply(&self, n: usize, alive: bool) -> Option<bool> {
        if alive {
            self.survives.get(n)
        } else {
            self.emerges.get(n)
        }
        .copied()
    }
}

impl Default for SimpleRules {
    fn default() -> Self {
        let mut survives: [bool; 7] = Default::default();

        survives[3] = true;
        survives[5] = true;

        let mut emerges: [bool; 7] = Default::default();

        emerges[2] = true;

        Self { survives, emerges }
    }
}

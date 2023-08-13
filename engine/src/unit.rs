use enum_iterator::Sequence;

use super::data;

#[derive(Clone, Copy, Sequence, PartialEq, Eq, Hash)]
pub enum Grassness {
    Poor,
    Usual,
    Rich,
}

#[derive(Clone, Copy)]
pub struct UnitData {
    pub inhabited: bool,
    pub grass: Grassness,
    pub marked: bool,
}

impl UnitData {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self {
            inhabited: false,
            grass: Grassness::Usual,
            marked: false,
        }
    }

    pub fn with_added_life(mut self) -> Self {
        self.inhabited = true;
        self
    }

    pub fn with_removed_life(mut self) -> Self {
        self.inhabited = false;
        self
    }

    pub fn add_life(&mut self) {
        self.inhabited = true;
    }

    pub fn remove_life(&mut self) {
        self.inhabited = false;
    }

    pub fn add_grass(mut self) -> Self {
        self.grass = self.grass.next().unwrap_or(self.grass);
        self
    }

    pub fn with_mark(mut self) -> Self {
        self.marked = true;
        self
    }

    pub fn with_removed_marked(mut self) -> Self {
        self.marked = false;
        self
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn unmark(&mut self) {
        self.marked = false;
    }

    pub fn remove_grass(mut self) -> Self {
        self.grass = self.grass.previous().unwrap_or(self.grass);
        self
    }

    pub fn randomize_life(&mut self, p: f64) {
        let inhabited = rand::random::<u32>() % 100 < ((100.0 * p.abs()) as u32);
        self.inhabited = inhabited;
    }

    pub fn compute_color(&self, is_penta: bool) -> [f32; 4] {
        let base_color = if self.inhabited {
            data::UNIT_COLOR
        } else if is_penta {
            data::ANOTHER_BACK_COLOR
        } else {
            data::BACK_COLOR
        };

        let grass_color = match self.grass {
            Grassness::Rich => data::GRASS_COLOR,
            Grassness::Usual => data::BACK_COLOR,
            Grassness::Poor => data::SCORCHD_COLOR,
        };

        let color = merge_colors(&base_color, &grass_color);

        if self.marked {
            let marked_color = [1.0, 0.0, 0.0, 0.9];
            merge_colors(&marked_color, &color)
        } else {
            color
        }
    }
}

impl Default for UnitData {
    fn default() -> Self {
        Self::new()
    }
}

fn merge_colors(lhs: &[f32; 4], rhs: &[f32; 4]) -> [f32; 4] {
    let [r1, g1, b1, a1] = lhs;
    let [r2, g2, b2, a2] = rhs;

    [
        (r1 + r2) / 2.0,
        (g1 + g2) / 2.0,
        (b1 + b2) / 2.0,
        (a1 + a2) / 2.0,
    ]
}

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
}

impl UnitData {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self {
            inhabited: false,
            grass: Grassness::Usual,
        }
    }

    pub fn add_life(mut self) -> Self {
        self.inhabited = true;
        self
    }

    pub fn remove_life(mut self) -> Self {
        self.inhabited = false;
        self
    }

    pub fn add_grass(mut self) -> Self {
        self.grass = self.grass.next().unwrap_or(self.grass);
        self
    }

    pub fn remove_grass(mut self) -> Self {
        self.grass = self.grass.previous().unwrap_or(self.grass);
        self
    }

    pub fn randomize_life(&mut self, p: f64) {
        let inhabited = rand::random::<u32>() % 100 < ((100.0 * p.abs()) as u32);
        self.inhabited = inhabited;
    }

    pub fn compute_color(&self) -> [f32; 4] {
        // BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)
        let base_color = if self.inhabited {
            data::UNIT_COLOR
        } else {
            data::BACK_COLOR
        };

        let grass_color = match self.grass {
            Grassness::Rich => data::GRASS_COLOR,
            Grassness::Usual => data::BACK_COLOR,
            Grassness::Poor => data::SCORCHD_COLOR,
        };

        merge_colors(&base_color, &grass_color)
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

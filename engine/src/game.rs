use std::collections::HashMap;

use crate::unit::{Grassness, UnitData};

pub struct Field(pub HashMap<SphericalIndex, UnitData>);
pub struct Game {
    pub present: Field,
    pub future: Field,
}

impl Game {
    pub fn new() -> Self {
        Self {
            present: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
            future: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
        }
    }

    pub fn next_tick(&mut self) {
        self.present
            .0
            .iter()
            .map(|(idx, data)| {
                (
                    Unit {
                        backref: &self.present,
                        idx: *idx,
                        data: *data,
                    }
                    .transform(),
                    *idx,
                )
            })
            .for_each(|(u, idx)| {
                self.future.0.insert(idx, u);
            });
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.present, &mut self.future);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Unit<'a> {
    backref: &'a Field,
    pub idx: SphericalIndex,
    pub data: UnitData,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct SphericalIndex(pub h3o::CellIndex);

impl<'a> Unit<'a> {
    pub fn get_neighbours(&self) -> impl Iterator<Item = UnitData> + '_ {
        self.idx
            .0
            .grid_disk::<Vec<_>>(1)
            .into_iter()
            .filter(|idx| *idx != self.idx.0)
            .map(|idx| *self.backref.0.get(&SphericalIndex(idx)).unwrap())
    }

    pub fn transform(&self) -> UnitData {
        let n: usize = self.get_neighbours().filter(|n| n.inhabited).count();

        match (self.data.grass, self.data.inhabited) {
            (Grassness::Rich, _) if n == 3 || n == 5 => self.data,
            (Grassness::Rich, false) if n == 1 || n == 2 => self.data.add_life(),

            (Grassness::Usual, _) if n == 3 || n == 5 => self.data,
            (Grassness::Usual, false) if n == 2 => self.data.add_life(),

            (Grassness::Poor, true) if n >= 2 || n <= 3 => self.data,
            (Grassness::Poor, false) if n == 5 => self.data.add_life(),

            _ => self.data.remove_life(),
        }
    }
}

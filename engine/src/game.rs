use std::collections::HashMap;

use h3o::CellIndex;

use crate::unit::{Grassness, UnitData};

pub struct Field(pub HashMap<SphericalIndex, UnitData>);
pub struct Game {
    pub present: Field,
    pub future: Field,

    pub indecies: Vec<h3o::CellIndex>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            present: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
            future: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
            indecies: h3o::CellIndex::base_cells()
                .flat_map(|index| index.children(crate::data::RESOLUTION))
                .collect::<Vec<_>>(),
        }
    }

    pub fn with_spawned_life(mut self) -> Self {
        self.spawn_life();
        self
    }

    pub fn spawn_life(&mut self) {
        self.indecies.iter().for_each(|index| {
            let mut unit = UnitData::new();
            unit.randomize_life(0.5);

            self.present.0.insert(SphericalIndex(*index), unit);
            self.future.0.insert(SphericalIndex(*index), unit);
        });
    }

    pub fn kill_everything(&mut self) {
        self.indecies.iter().for_each(|index| {
            let unit = self.present.0.get_mut(&SphericalIndex(*index)).unwrap();
            *unit = unit.with_removed_life();
        });
    }

    pub fn remove_marks(&mut self) {
        self.present.0.iter_mut().for_each(|(_k, v)| {
            v.unmark();
        });
    }

    pub fn unmark_unit(&mut self, index: h3o::CellIndex) {
        self.present
            .0
            .get_mut(&SphericalIndex(index))
            .iter_mut()
            .for_each(|u| u.unmark());
    }

    pub fn mark_unit(&mut self, index: h3o::CellIndex) {
        self.present
            .0
            .get_mut(&SphericalIndex(index))
            .iter_mut()
            .for_each(|u| u.mark());
    }

    pub fn get_mut_unit(&mut self, index: &h3o::CellIndex) -> Option<&mut UnitData> {
        self.present.0.get_mut(&SphericalIndex(*index))
    }

    pub fn get_unit(&self, index: h3o::CellIndex) -> Option<&UnitData> {
        self.present.0.get(&SphericalIndex(index))
    }

    pub fn get_raw_coords(&self, index: h3o::CellIndex) -> (f64, f64) {
        let sph = h3o::LatLng::from(index);
        (sph.lat_radians(), sph.lng_radians())
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

    pub fn cell_to_colored_face_vtxes(
        &self,
        index: &h3o::CellIndex,
    ) -> std::vec::Vec<((f64, f64, f64), [f32; 4])> {
        let color = self.get_unit(*index).unwrap().compute_color();
        let boundary = index.boundary();
        boundary
            .iter()
            .zip(boundary.iter().cycle().skip(1))
            .zip([h3o::LatLng::from(*index)].into_iter().cycle())
            .flat_map(|((i, j), c)| {
                [as_cartesian(i), as_cartesian(j), as_cartesian(&c)].into_iter()
            })
            .zip([color].into_iter().cycle())
            .collect()
    }

    pub fn get_mesh_parts(&self) {}
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
            (Grassness::Rich, false) if n == 1 || n == 2 => self.data.with_added_life(),

            (Grassness::Usual, _) if n == 3 || n == 5 => self.data,
            (Grassness::Usual, false) if n == 2 => self.data.with_added_life(),

            (Grassness::Poor, true) if n >= 2 || n <= 3 => self.data,
            (Grassness::Poor, false) if n == 5 => self.data.with_added_life(),

            _ => self.data.with_removed_life(),
        }
    }
}

pub fn as_cartesian(ltln: &h3o::LatLng) -> (f64, f64, f64) {
    let (lt, ln) = (ltln.lat_radians(), ltln.lng_radians());
    (lt.cos() * ln.cos(), lt.cos() * ln.sin(), lt.sin())
}

pub fn as_spherical((x, y, z): &(f64, f64, f64)) -> h3o::LatLng {
    let lat = z.asin();
    let lng = y.atan2(*x);
    h3o::LatLng::from_radians(lat, lng).unwrap()
}

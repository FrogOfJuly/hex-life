use std::collections::HashMap;

use h3o::{LatLng, Resolution};

use crate::{rules::SimpleRules, unit::UnitData};

pub struct Field(pub HashMap<SphericalIndex, UnitData>);
pub struct Game {
    pub present: Field,
    pub future: Field,

    pub indecies: Vec<h3o::CellIndex>,
    pub resolution: h3o::Resolution,
}

impl Game {
    pub fn new(&resolution: &h3o::Resolution) -> Self {
        Self {
            present: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
            future: Field(std::collections::HashMap::<SphericalIndex, UnitData>::new()),
            indecies: h3o::CellIndex::base_cells()
                .flat_map(|index| index.children(resolution))
                .collect::<Vec<_>>(),
            resolution,
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

    pub fn next_tick(&mut self, rules: &SimpleRules) {
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
                    .transform(rules),
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
        let color = self
            .get_unit(*index)
            .unwrap()
            .compute_color(index.is_pentagon());
        let boundary: Vec<_> = index.vertexes().collect();
        if index.is_pentagon() {
            &[0, 1, 4, 1, 2, 4, 2, 3, 4][..]
        } else {
            &[0, 2, 4, 2, 3, 4, 0, 1, 2, 0, 4, 5][..]
        }
        
        .iter()
        .map(|&i| &boundary[i as usize])
        .map(|&x| LatLng::from(x))
        .map(|ltln| as_cartesian(&ltln))
        .zip([color].into_iter().cycle())
        .collect()
        // boundary
        //     .iter()
        //     .zip(boundary.iter().cycle().skip(1))
        //     .zip([h3o::LatLng::from(*index)].into_iter().cycle())
        //     .flat_map(|((i, j), c)| {
        //         [as_cartesian(i), as_cartesian(j), as_cartesian(&c)].into_iter()
        //     })
        //     .zip([color].into_iter().cycle())
        //     .collect()
    }

    pub fn decrease_fineness(&mut self) {
        if let Some(resolution) = dec_resolution(&self.resolution) {
            *self = Game::new(&resolution).with_spawned_life();
        }
    }

    pub fn increase_fineness(&mut self) {
        if let Some(resolution) = inc_resolution(&self.resolution) {
            *self = Game::new(&resolution).with_spawned_life();
        }
    }
}

pub fn as_number(r: &h3o::Resolution) -> u32 {
    match r {
        Resolution::Zero => 0,
        Resolution::One => 1,
        Resolution::Two => 2,
        Resolution::Three => 3,
        Resolution::Four => 4,
        Resolution::Five => 5,
        Resolution::Six => 6,
        Resolution::Seven => 7,
        Resolution::Eight => 8,
        Resolution::Nine => 9,
        Resolution::Ten => 10,
        Resolution::Eleven => 11,
        Resolution::Twelve => 12,
        Resolution::Thirteen => 13,
        Resolution::Fourteen => 14,
        Resolution::Fifteen => 15,
    }
}

fn as_resolution(i: u32) -> Option<h3o::Resolution> {
    match i {
        0 => Some(Resolution::Zero),
        1 => Some(Resolution::One),
        2 => Some(Resolution::Two),
        3 => Some(Resolution::Three),
        4 => Some(Resolution::Four),
        5 => Some(Resolution::Five),
        6 => Some(Resolution::Six),
        7 => Some(Resolution::Seven),
        8 => Some(Resolution::Eight),
        9 => Some(Resolution::Nine),
        10 => Some(Resolution::Ten),
        11 => Some(Resolution::Eleven),
        12 => Some(Resolution::Twelve),
        13 => Some(Resolution::Thirteen),
        14 => Some(Resolution::Fourteen),
        15 => Some(Resolution::Fifteen),
        _ => None,
    }
}

fn inc_resolution(r: &h3o::Resolution) -> Option<h3o::Resolution> {
    as_resolution(as_number(r) + 1)
}

fn dec_resolution(r: &h3o::Resolution) -> Option<h3o::Resolution> {
    if as_number(r) == 0 {
        None
    } else {
        as_resolution((as_number(r) as i32 - 1) as u32)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(&h3o::Resolution::Two)
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

    pub fn transform(&self, rules: &SimpleRules) -> UnitData {
        let n: usize = self.get_neighbours().filter(|n| n.inhabited).count();

        self.data
            .with_set_life(rules.apply(n, self.data.inhabited).unwrap())
    }
}

pub fn as_cartesian(ltln: &h3o::LatLng) -> (f64, f64, f64) {
    let (lt, ln) = (ltln.lat_radians(), ltln.lng_radians());
    (lt.cos() * ln.cos(), lt.cos() * ln.sin(), lt.sin())
}

pub fn as_spherical((x, y, z): &(f64, f64, f64)) -> Option<h3o::LatLng> {
    let lat = z.asin();
    let lng = y.atan2(*x);
    h3o::LatLng::from_radians(lat, lng).ok()
}

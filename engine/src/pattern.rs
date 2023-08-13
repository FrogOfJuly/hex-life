use std::{collections::HashMap, str::FromStr, vec};

use h3o::CellIndex;

// pub struct Pattern {
//     resolution: h3o::Resolution,
//     // center: h3o::CellIndex,
//     cells: Vec<h3o::CellIndex>,
// }

pub trait Pattern {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex>;
}

struct SingleCell;

impl Pattern for SingleCell {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        vec![*center]
    }
}

struct Star;

impl Pattern for Star {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        center
            .grid_disk::<Vec<_>>(1)
            .into_iter()
            .filter(|idx| idx != center)
            .collect()
    }
}

struct SmallPulsar {
    pattern: [CellIndex; 3],
}

impl SmallPulsar {
    fn new() -> Self {
        Self {
            pattern: [
                h3o::CellIndex::try_from(0x8201a7fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x8201affffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x820117fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for SmallPulsar {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern)
    }
}

fn transpose_pattern(src: CellIndex, dst: CellIndex, pattern: &[CellIndex]) -> Vec<CellIndex> {
    // Compute translation offset.
    let src_coord = src.to_local_ij(src).expect("src coord");
    let dst_coord = dst.to_local_ij(dst).expect("dst coord");
    let i_offset = dst_coord.i() - src_coord.i();
    let j_offset = dst_coord.j() - src_coord.j();

    // Transpose the pattern from src to dst.
    pattern
        .iter()
        .copied()
        .map(|cell| {
            // Compute the local IJ coordinate wrt original center cell.
            let src_ij = cell.to_local_ij(src).expect("local IJ");
            // Apply translation and re-anchor at destination center cell.
            let dst_ij =
                h3o::LocalIJ::new_unchecked(dst, src_ij.i() + i_offset, src_ij.j() + j_offset);
            // Convert back to cell index.
            CellIndex::try_from(dst_ij).expect("dst cell")
        })
        .collect::<Vec<_>>()
}

// impl Pattern {

//     pub fn as_cells(&self, index: h3o::CellIndex) -> Vec<h3o::CellIndex> {
//         self.cells.clone()
//     }

//     pub fn single_cell() -> Self {
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: vec![(0.0, 0.0)],
//         }
//     }

//     pub fn pulsating_trio() -> Self {
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: Self::from_probe(&[
//                 (0.628258801974152, 0.028561568592762487),
//                 (0.628258801974152, 0.028561568592762487),
//                 (0.6752809643597009, 0.008259692499186344),
//                 (0.6359831161201062, -0.03016146339908843),
//             ]),
//         }
//     }

//     pub fn rotating_trio() -> Self {
//         //broken
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: Self::from_probe(&[
//                 (1.3706529721123182, 1.9630044225232948),
//                 (1.3166635884093614, 1.970179801013055),
//                 (1.393610190564252, 2.22453727062287),
//                 (1.3896366500190047, 1.700404701470688),
//                 (1.3706529721123182, 1.9630044225232948),
//             ]),
//         }
//     }

//     pub fn small_flicker() -> Self {
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: vec![(0.0, 0.0), (-0.048_393_81, 0.088_904_33)],
//         }
//     }

//     pub fn large_flicker() -> Self {
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: Self::from_probe(&[
//                 (0.3952834196131237, -0.6040611734833741),
//                 (0.3548937048005787, -0.5680108266491317),
//                 (0.36130498910826964, -0.512373079161566),
//                 (0.4089842832179954, -0.48991601364639653),
//                 (0.4514036422259739, -0.525592480324781),
//                 (0.4514036422259739, -0.525592480324781),
//                 (0.4440631446401373, -0.5842605758240044),
//             ]),
//         }
//     }

//     pub fn blob() -> Self {
//         Self {
//             //broken
//             resolution: h3o::Resolution::Two,
//             cells: Self::from_probe(&[
//                 (0.2003686623800157, -1.2360278420245137),
//                 (0.15678260655594517, -1.3060826685358629),
//                 (0.2392512364430469, -1.2653181832792098),
//                 (0.2013608762439863, -1.1877619795012608),
//                 (0.16064691788507196, -1.2067039997019218),
//             ]),
//         }
//     }

//     pub fn big_pulsar() -> Self {
//         //works?
//         Self {
//             resolution: h3o::Resolution::Two,
//             cells: Self::from_probe(&[
//                 (1.4777717872212457, 1.9244906553138363),
//                 (1.424468195197057, 1.9506680350714976),
//                 (1.4398053799276007, 1.5838016468314247),
//                 (1.4856622748805157, 1.3375929608900674),
//                 (1.5300953499615786, 1.8318968335023422),
//                 (1.4945676365071172, 2.5387804146686057),
//                 (1.4456860505734481, 2.319685833801514),
//             ]),
//         }
//     }

// }
pub fn create_pattern_map() -> HashMap<&'static str, Box<dyn Pattern>> {
    let mut patterns: HashMap<&'static str, Box<dyn Pattern>> = HashMap::new();

    patterns.insert("Single cell", Box::new(SingleCell));
    patterns.insert("Star", Box::new(Star));
    patterns.insert("Small pulsar", Box::new(SmallPulsar::new()));
    // patterns.insert("Rotating trio", Box::new(Pattern::rotating_trio));
    // patterns.insert("Pulsating trio", Box::new(Pattern::pulsating_trio));
    // patterns.insert("Blob", Box::new(Pattern::blob));
    // patterns.insert("Big pulsar", Box::new(Pattern::big_pulsar));

    patterns
}

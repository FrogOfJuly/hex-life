use std::{collections::HashMap, vec};

use h3o::CellIndex;

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

struct MediumWiggler {
    pattern: [CellIndex; 4],
}

impl MediumWiggler {
    fn new() -> Self {
        Self {
            pattern: [
                h3o::CellIndex::try_from(0x827c67fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x827c77fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x827c57fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x827c5ffffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for MediumWiggler {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern)
    }
}

struct SmallFlicker {
    pattern: [CellIndex; 2],
}

impl SmallFlicker {
    fn new() -> Self {
        Self {
            pattern: [
                h3o::CellIndex::try_from(0x8208a7fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x820837fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for SmallFlicker {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern)
    }
}

struct RotatingTrio {
    pattern: [CellIndex; 4],
}

impl RotatingTrio {
    fn new() -> Self {
        Self {
            //821317fffffffff
            pattern: [
                h3o::CellIndex::try_from(0x821317fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x8213a7fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82131ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821337fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for RotatingTrio {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern[1..])
    }
}

struct Blob {
    pattern: [CellIndex; 7],
}

impl Blob {
    fn new() -> Self {
        Self {
            //821317fffffffff
            pattern: [
                h3o::CellIndex::try_from(0x82c067fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82c147fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82c14ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82ad6ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82c077fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82c047fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82d137fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for Blob {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern[1..])
    }
}



struct LittleBlob {
    pattern: [CellIndex; 5],
}

impl LittleBlob {
    fn new() -> Self {
        Self {
            //821317fffffffff
            pattern: [
                h3o::CellIndex::try_from(0x82a627fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82a62ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82a777fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82a71ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82a637fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for LittleBlob {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern[1..])
    }
}


struct Glider {
    pattern: [CellIndex; 11],
}

impl Glider {
    fn new() -> Self {
        Self {
            //821317fffffffff
            pattern: [
                h3o::CellIndex::try_from(0x82130ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821327fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x8202dffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821347fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x82122ffffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821237fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821317fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x8212affffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x8212e7fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821267fffffffff).expect("Invalid literal for cell"),
                h3o::CellIndex::try_from(0x821387fffffffff).expect("Invalid literal for cell"),
            ],
        }
    }
}

impl Pattern for Glider {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        transpose_pattern(self.pattern[0], *center, &self.pattern[1..])
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

pub fn create_pattern_map() -> HashMap<&'static str, Box<dyn Pattern>> {
    let mut patterns: HashMap<&'static str, Box<dyn Pattern>> = HashMap::new();

    patterns.insert("Single cell", Box::new(SingleCell));
    patterns.insert("Star", Box::new(Star));
    patterns.insert("Small pulsar", Box::new(SmallPulsar::new()));
    patterns.insert("Small flicker", Box::new(SmallFlicker::new()));
    patterns.insert("Rotating trio", Box::new(RotatingTrio::new()));
    patterns.insert("Medium wiggler", Box::new(MediumWiggler::new()));
    patterns.insert("Blob", Box::new(Blob::new()));
    patterns.insert("Little blob", Box::new(LittleBlob::new()));

    patterns
}

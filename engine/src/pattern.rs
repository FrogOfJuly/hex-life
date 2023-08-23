use std::{collections::HashMap, vec};

use h3o::CellIndex;

pub trait Pattern {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex>;

    fn size(&self) -> usize;
}

struct SingleCell;

impl Pattern for SingleCell {
    fn as_cells(&self, center: &h3o::CellIndex) -> Vec<h3o::CellIndex> {
        vec![*center]
    }

    fn size(&self) -> usize {
        1
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

    fn size(&self) -> usize {
        7
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

    fn size(&self) -> usize {
        3
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

    fn size(&self) -> usize {
        4
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

    fn size(&self) -> usize {
        2
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

    fn size(&self) -> usize {
        3
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

    fn size(&self) -> usize {
        6
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

    fn size(&self) -> usize {
        4
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

    fn size(&self) -> usize {
        10
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

pub fn create_pattern_map() -> Vec<(&'static str, Box<dyn Pattern>)> {
    let mut pattern_map: HashMap<&'static str, Box<dyn Pattern>> = HashMap::new();

    pattern_map.insert("Single cell", Box::new(SingleCell));
    pattern_map.insert("Star", Box::new(Star));
    pattern_map.insert("Small pulsar", Box::new(SmallPulsar::new()));
    pattern_map.insert("Small flicker", Box::new(SmallFlicker::new()));
    pattern_map.insert("Rotating trio", Box::new(RotatingTrio::new()));
    pattern_map.insert("Medium wiggler", Box::new(MediumWiggler::new()));
    pattern_map.insert("Blob", Box::new(Blob::new()));
    pattern_map.insert("Little blob", Box::new(LittleBlob::new()));
    pattern_map.insert("Glider", Box::new(Glider::new()));

    let mut patterns: Vec<_> = pattern_map.into_iter().collect();

    patterns.sort_by(|(k1, p1), (k2, p2)| {
        if p1.size() != p2.size(){
            p1.size().cmp(&p2.size())
        }else{
            k1.cmp(k2)
        }
    });

    patterns
}

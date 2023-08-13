pub struct Pattern {
    resolution: h3o::Resolution,
    cells: Vec<(f64, f64)>,
}

impl Pattern {
    pub fn as_cells(&self, index: h3o::CellIndex) -> Vec<h3o::CellIndex> {
        self.cells
            .iter()
            .map(|(lat_diff, lng_diff)| {
                let (lat, lng) = {
                    let ltln = h3o::LatLng::from(index);
                    (ltln.lat_radians(), ltln.lng_radians())
                };
                h3o::LatLng::from_radians(lat + lat_diff, lng + lng_diff)
                    .unwrap()
                    .to_cell(self.resolution)
            })
            .collect()
    }

    pub fn single_cell() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: vec![(0.0, 0.0)],
        }
    }

    pub fn pulsating_trio() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: vec![],
        }
    }

    pub fn rotating_trio() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: vec![],
        }
    }

    //[(1.2593540769381733, 2.6258493386769572), (1.3077478834556382, 2.5369450098779214)]
    pub fn small_flicker() -> Self {
        //1.2332145070283227
        Self {
            resolution: h3o::Resolution::Two,
            cells: vec![(0.0, 0.0), (-0.048_393_81, 0.088_904_33)],
        }
    }

    pub fn large_flicker() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: vec![],
        }
    }
}

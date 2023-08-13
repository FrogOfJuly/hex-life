use std::vec;

pub struct Pattern {
    resolution: h3o::Resolution,
    cells: Vec<(f64, f64)>,
}

impl Pattern {
    fn from_probe(probe: &[(f64, f64)]) -> Vec<(f64, f64)> {
        probe
            .iter()
            .skip(1)
            .map(|(x, y)| {
                let (cx, cy) = probe[0];
                (x - cx, y - cy)
            })
            .collect()
    }

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
            cells: Self::from_probe(&[
                (0.628258801974152, 0.028561568592762487),
                (0.628258801974152, 0.028561568592762487),
                (0.6752809643597009, 0.008259692499186344),
                (0.6359831161201062, -0.03016146339908843),
            ]),
        }
    }

    pub fn rotating_trio() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: Self::from_probe(&[
                (1.3706529721123182, 1.9630044225232948),
                (1.3166635884093614, 1.970179801013055),
                (1.393610190564252, 2.22453727062287),
                (1.3896366500190047, 1.700404701470688),
                (1.3706529721123182, 1.9630044225232948),
            ]),
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
            cells: Self::from_probe(&[
                (0.956163014320513, -3.0425256703555488),
                (0.956163014320513, -3.0425256703555488),
                (0.9582043732122311, 3.090758763546036),
                (0.8815386091817468, 3.029874856384683),
            ]),
        }
    }

    pub fn blob() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: Self::from_probe(&[
                (0.2003686623800157, -1.2360278420245137),
                (0.15678260655594517, -1.3060826685358629),
                (0.2392512364430469, -1.2653181832792098),
                (0.2013608762439863, -1.1877619795012608),
                (0.16064691788507196, -1.2067039997019218),
            ]),
        }
    }

    pub fn big_pulsar() -> Self {
        Self {
            resolution: h3o::Resolution::Two,
            cells: Self::from_probe(&[
                (0.3952834196131237, -0.6040611734833741),
                (0.3548937048005787, -0.5680108266491317),
                (0.36130498910826964, -0.512373079161566),
                (0.4089842832179954, -0.48991601364639653),
                (0.4514036422259739, -0.525592480324781),
                (0.4514036422259739, -0.525592480324781),
                (0.4440631446401373, -0.5842605758240044),
            ]),
        }
    }
}

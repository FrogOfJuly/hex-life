use std::collections::HashMap;
use enum_iterator::Sequence;

use three_d::*;

mod data {
    pub const BACK_COLOR: [f32; 4] = [0.204, 0.286, 0.369, 1.0];
    pub const UNIT_COLOR: [f32; 4] = [0.1, 0.9, 0.1, 0.3];
    pub const GRASS_COLOR: [f32; 4] = [0.4, 0.9, 0.1, 1.0];
    pub const SCORCHD_COLOR: [f32; 4] = [0.9, 0.4, 0.1, 1.0];
    pub const BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
}


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

fn merge_colors(
    lhs: &[f32;4],
    rhs: &[f32;4],
) -> [f32;4] {
    let [r1, g1, b1, a1] = lhs;
    let [r2, g2, b2, a2] = rhs;

    [
        (r1 + r2) / 2.0,
        (g1 + g2) / 2.0,
        (b1 + b2) / 2.0,
        (a1 + a2) / 2.0,
    ]
}



struct Field(HashMap<SphericalIndex, UnitData>);
struct Game {
    present: Field,
    future: Field,
}

impl Game {
    fn new() -> Self {
        Self {
            present: Field(std::collections::HashMap::<
                SphericalIndex,
                UnitData,
            >::new()),
            future: Field(std::collections::HashMap::<
                SphericalIndex,
                UnitData,
            >::new()),
        }
    }

    fn next_tick(&mut self) {
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

    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.present, &mut self.future);
    }
}

struct Unit<'a> {
    backref: &'a Field,
    idx: SphericalIndex,
    data: UnitData,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct SphericalIndex(h3o::CellIndex);

impl<'a> Unit<'a> {
    fn get_neighbours(&self) -> impl Iterator<Item = UnitData> + '_ {
        self.idx
            .0
            .grid_disk::<Vec<_>>(1)
            .into_iter()
            .filter(|idx| *idx != self.idx.0)
            .map(|idx| *self.backref.0.get(&SphericalIndex(idx)).unwrap())
    }

    fn transform(&self) -> UnitData {
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


fn lnlt_to_unit_sphere(ltln: &h3o::LatLng) -> (f64, f64, f64) {
    let (lt, ln) = (ltln.lat_radians(), ltln.lng_radians());
    (lt.cos() * ln.cos(), lt.cos() * ln.sin(), lt.sin())
}

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Life".to_string(),
        max_size: Some((900, 900)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let mut gui = three_d::GUI::new(&context);
    

    let mut game = Game::new();    
    let resolution = h3o::Resolution::Two;

    let indecies = h3o::CellIndex::base_cells()
    .flat_map(|index| index.children(resolution)).collect::<Vec<_>>();

    indecies.iter()
                .for_each(|index|{
                    let mut unit = UnitData::new();
                    unit.randomize_life(0.5);
        
                    game.present.0.insert(SphericalIndex(*index), unit);
                    game.future.0.insert(SphericalIndex(*index), unit);
                });

    let mut pause = false;
    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        frame_input.events.iter().for_each(|event|{
            let speed = 0.5;
            if let Event::KeyPress { kind, ..} = event {
                match kind {
                    Key::ArrowDown => camera.rotate_around(&Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 0.0, -speed),
                    Key::ArrowLeft => camera.rotate_around(&Vector3 { x: 0.0, y: 0.0, z: 0.0 }, -speed, 0.0),
                    Key::ArrowRight => camera.rotate_around(&Vector3 { x: 0.0, y: 0.0, z: 0.0 }, speed, 0.0),
                    Key::ArrowUp => camera.rotate_around(&Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 0.0, speed),
                    Key::Enter => {pause = !pause;},
                    _ => ()
                }
            }
        });
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        let (vtxes, colors)  = indecies.iter()
                .map(|index| (index, lnlt_to_unit_sphere(&h3o::LatLng::from(*index))))
                .flat_map(|(index, center)| {    
                    let boundary = index.boundary();
                    boundary.iter()
                            .zip(boundary.iter().cycle().skip(1))
                            .zip([center].into_iter().cycle())
                            .flat_map(|((i, j), c)| 
                                [ lnlt_to_unit_sphere(i)
                                , lnlt_to_unit_sphere(j)
                                , c].into_iter()
                            )
                            .map(|(x, y, z)| Vector3{x, y ,z})
                            .zip({
                                let unit = game.present.0.get(&SphericalIndex(*index)).unwrap();
                                let [r, g, b, a] = unit.compute_color();
                                let tr = |x : f32| {
                                    let f2 = x.min(1.0).max(0.0);
                                    if f2 == 1.0 { 255 } else{ (f2 * 256.0) as u8}
                                };
                                [Srgba{r: tr(r), g : tr(g), b : tr(b), a : tr(a)}]
                            }.into_iter().cycle()).collect::<Vec<_>>()

                }).unzip();

        let model = Gm::new(Mesh::new(&context, &CpuMesh{
            positions : Positions::F64(vtxes) ,
            colors : Some(colors),
            ..Default::default()
        }), ColorMaterial::default());
        
        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model, &[]
            );

        game.next_tick();
        if !pause{
            game.swap_buffers();
        }

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}

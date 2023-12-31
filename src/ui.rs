use std::collections::VecDeque;

use engine::game::as_number;
use three_d::{Camera, OrbitControl};

pub struct GUIState {
    pub pause: bool,
    pub skip_frame: bool,
    patterns: Vec<(&'static str, Box<dyn engine::pattern::Pattern>)>,
    pub toggled_pattern: Option<&'static str>,
    pub rules: engine::rules::SimpleRules,
    pub orbit_control: OrbitControl,
    pub fps: VecDeque<f64>,
    pub time_beg: Option<f64>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
export function performance_now() {
  return performance.now();
}"#)]
extern "C" {
    fn performance_now() -> f64;
}

impl GUIState {
    #[cfg(target_arch = "wasm32")]
    const FRAME_INTERVAL: usize = 10;

    pub fn new(camera: &Camera) -> Self {
        Self {
            pause: false,
            skip_frame: false,
            patterns: engine::pattern::create_pattern_map(),
            toggled_pattern: None,
            rules: engine::rules::SimpleRules::default(),
            orbit_control: OrbitControl::new(*camera.target(), 1.0, 100.0),
            fps: VecDeque::new(),
            time_beg: None,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_fps(&self) -> f64 {
        let seconds = {
            let ms: f64 = self.fps.iter().cloned().sum();
            ms / 1000.0
        };

        log::info!(
            "fps = {:?}/{:?}, frames: {:?}",
            self.fps.len() as f64,
            seconds,
            self.fps
        );
        self.fps.len() as f64 / seconds
    }

    #[cfg(target_arch = "wasm32")]
    pub fn record_fps(&mut self) {
        if let Some(beg) = self.time_beg {
            self.fps.push_back(performance_now() - beg);

            if self.fps.len() > Self::FRAME_INTERVAL as usize {
                self.fps.pop_front();
            }
        }
        self.time_beg = Some(performance_now())
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn draw_ui(&mut self, gui_context: &three_d::egui::Context, game: &mut engine::game::Game) {
        use three_d::egui::*;

        #[cfg(target_arch = "wasm32")]
        self.record_fps();

        Window::new("Game of life")
            .movable(false)
            .collapsible(true)
            .show(gui_context, |ui| {
                ui.label(" ");

                ui.hyperlink_to("Github", "https://frogofjuly.github.io/hex-life");

                #[cfg(target_arch = "wasm32")]
                ui.label(format!("FPS: {:.1}", self.get_fps()));

                ui.label(" ");

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Controls");

                        if ui
                            .add(Button::new(if self.pause { "Run  " } else { "Pause" }))
                            .clicked()
                        {
                            self.pause = !self.pause;
                        }
                        if ui.add(Button::new("Clear")).clicked() {
                            game.kill_everything();
                        }

                        if ui.add(Button::new("Fill")).clicked() {
                            game.spawn_life();
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading(format!("Grid fineness: {:?}", as_number(&game.resolution)));

                        if ui.add(Button::new("Increase")).clicked() {
                            game.increase_fineness();
                        }

                        if ui.add(Button::new("Decrease")).clicked() {
                            game.decrease_fineness();
                        }
                    });
                });

                ui.separator();

                ui.heading("Patterns");
                ui.label("Choose pattern:");

                self.patterns.iter().for_each(|(k, _build_pattern)| {
                    let mut b = Button::new(k.to_string());
                    if let Some(pattern_name) = self.toggled_pattern {
                        if pattern_name == *k {
                            b = b.fill(Color32::from_rgb(57, 115, 172))
                        }
                    };

                    match (ui.add(b).clicked(), self.toggled_pattern) {
                        (true, Some(toggled_pattern)) if toggled_pattern == *k => {
                            self.toggled_pattern = None;
                        }
                        (true, _) => self.toggled_pattern = Some(*k),
                        _ => (),
                    };
                });

                ui.label("Left-click to spawn");
                ui.label("");
                ui.separator();

                ui.label("* Use arrows or WASD to rotate the camera");
                ui.label("* Use Enter or Space to pause/unpause");
                ui.label("* Left-click on a sphere to mark a cell");
                ui.label("* Right-click on a sphere to kill a cell");

                ui.separator();

                ui.heading("Rules");

                ui.label("Survives");
                ui.horizontal(|ui| {
                    self.rules
                        .survives
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, v)| {
                            ui.add(Checkbox::new(v, i.to_string()));
                        });
                });

                ui.label("Emerges");
                ui.horizontal(|ui| {
                    self.rules
                        .emerges
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, v)| {
                            ui.add(Checkbox::new(v, i.to_string()));
                        });
                });

                ui.label(" ");
                ui.label(" ");
            });
    }

    pub fn handle_mouse_clicks(
        &mut self,
        geometry: impl IntoIterator<Item = impl three_d::Geometry>,
        camera: &three_d::Camera,
        context: &three_d::Context,
        game: &mut engine::game::Game,
        (position, button): (&three_d::LogicalPoint, &three_d::MouseButton),
    ) {
        if let Some(three_d::Vector3 { x, y, z }) =
            three_d::renderer::pick(context, camera, position, geometry)
        {
            log::info!("clicked here: {:?}", (x, y, z));
            let Some(index) = engine::game::as_spherical(&(x as f64, y as f64, z as f64))
                .map(|i| i.to_cell(game.resolution))
            else {
                log::info!("you clicked in a wrong place!");
                return;
            };

            if let (three_d::MouseButton::Left, Some(toggled_pattern)) =
                (button, self.toggled_pattern)
            {
                self.patterns
                    .iter()
                    .find(|(name, _pattern)| name == &toggled_pattern)
                    .iter()
                    .for_each(|(_name, u)| {
                        u.as_cells(&index).iter().for_each(|index| {
                            game.get_mut_unit(index)
                                .into_iter()
                                .for_each(|u| u.add_life());
                        })
                    });
            } else if let (three_d::MouseButton::Left, None) = (button, self.toggled_pattern) {
                game.get_mut_unit(&index)
                    .into_iter()
                    .for_each(|u| u.marked = !u.marked);

                log::info!("{:?}", index);
            } else if let three_d::MouseButton::Right = button {
                game.get_mut_unit(&index)
                    .into_iter()
                    .for_each(|u| u.remove_life());
            }

            self.skip_frame = true;
        } else {
            log::info!("click did not connect: {:?}", position);
        }
    }

    pub fn handle_keyboard_event(
        &mut self,
        camera: &mut three_d::Camera,
        kind: three_d::renderer::control::Key,
    ) {
        use three_d::renderer::control::Key;

        let speed = 0.5;

        match kind {
            Key::ArrowDown | Key::S => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                -speed,
            ),
            Key::ArrowLeft | Key::A => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                -speed,
                0.0,
            ),
            Key::ArrowRight | Key::D => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                speed,
                0.0,
            ),
            Key::ArrowUp | Key::W => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                speed,
            ),
            Key::Enter | Key::Space => self.toggle_pause(),
            _ => (),
        }
    }

    pub fn update_game_state(&mut self, game: &mut engine::game::Game) {
        if !self.pause && !self.skip_frame {
            game.next_tick(&self.rules);
            game.swap_buffers();
        }

        self.skip_frame = false;
    }
}

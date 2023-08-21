use std::collections::HashMap;

use engine::game::as_number;
use three_d::{Camera, OrbitControl};

pub struct GUIState {
    pub pause: bool,
    pub skip_frame: bool,
    patterns: HashMap<&'static str, Box<dyn engine::pattern::Pattern>>,
    pub toggled_pattern: Option<&'static str>,
    pub rules: engine::rules::SimpleRules,
    pub orbit_control: OrbitControl,
}

impl GUIState {
    pub fn new(camera: &Camera) -> Self {
        Self {
            pause: false,
            skip_frame: false,
            patterns: engine::pattern::create_pattern_map(),
            toggled_pattern: None,
            rules: engine::rules::SimpleRules::default(),
            orbit_control: OrbitControl::new(*camera.target(), 1.0, 100.0),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn draw_ui(&mut self, gui_context: &three_d::egui::Context, game: &mut engine::game::Game) {
        use three_d::egui::*;

        Window::new("Interface").show(gui_context, |ui| {
            ui.label(" ");

            ui.hyperlink_to("Github", "https://frogofjuly.github.io/hex-life");

            ui.label(" ");

            ui.heading("Controls");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
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
                self.patterns.get(toggled_pattern).iter().for_each(|u| {
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

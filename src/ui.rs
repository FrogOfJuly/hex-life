use std::collections::HashMap;

pub struct GUIState {
    pub pause: bool,
    pub skip_frame: bool,
    patterns: HashMap<&'static str, Box<dyn engine::pattern::Pattern>>,
    pub toggled_pattern: Option<&'static str>,
    pub rules: engine::rules::SimpleRules,
}

impl GUIState {
    pub fn new() -> Self {
        Self {
            pause: false,
            skip_frame: false,
            patterns: engine::pattern::create_pattern_map(),
            toggled_pattern: None,
            rules: engine::rules::SimpleRules::default(),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn draw_ui(&mut self, gui_context: &three_d::egui::Context, game: &mut engine::game::Game) {
        use three_d::egui::*;

        SidePanel::right("Controls").show(gui_context, |ui| {

            ui.label(" ");
            ui.label(" ");

            if ui.hyperlink_to("Github", "https://frogofjuly.github.io/hex-life").clicked(){
                
            };

            ui.label(" ");

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

            ui.separator();

            ui.heading("Patterns");

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

            ui.label("Choose pattern and left-click on the sphere to spawn it");
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
            let index = engine::game::as_spherical(&(x as f64, y as f64, z as f64))
                .to_cell(engine::data::RESOLUTION);

            if let (three_d::MouseButton::Left, Some(toggled_pattern)) =
                (button, self.toggled_pattern)
            {
                self.patterns
                    .get(toggled_pattern)
                    .unwrap()
                    .as_cells(&index)
                    .iter()
                    .for_each(|index| {
                        game.get_mut_unit(index)
                            .into_iter()
                            .for_each(|u| u.add_life());
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

pub struct GUIState {
    pub pause: bool,
    pub skip_frame: bool,
    patterns: std::collections::HashMap<&'static str, Box<dyn Fn() -> engine::pattern::Pattern>>,
    pub msges: Vec<(f64, f64)>,
}

impl GUIState {
    pub fn new() -> Self {
        Self {
            pause: true,
            skip_frame: false,
            patterns: engine::pattern::Pattern::create_pattern_map(),
            msges: vec![],
        }
    }

    pub fn toogle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn draw_ui(&mut self, gui_context: &three_d::egui::Context, game: &mut engine::game::Game) {
        use three_d::egui::*;

        SidePanel::left("Controls").show(gui_context, |ui| {
            ui.label("Controls");
            if ui
                .add(Button::new(if self.pause { "Run  " } else { "Pause" }))
                .clicked()
            {
                self.pause = !self.pause;
            }
            if ui.add(Button::new("Clear")).clicked() {
                game.kill_everything();
            }
            if ui.add(Button::new("Clear marks")).clicked() {
                game.remove_marks();
                self.msges.clear();
            }
            if ui.add(Button::new("Fill")).clicked() {
                game.spawn_life();
            }

            ui.separator();

            ui.label("Patterns");

            self.patterns.iter().for_each(|(k, build_pattern)| {
                if ui.add(Button::new(k.to_string())).clicked() {
                    let indecies = game
                        .present
                        .0
                        .iter()
                        .filter(|(_k, v)| v.marked)
                        .map(|(k, _)| k.0)
                        .flat_map(|index| build_pattern().as_cells(index))
                        .collect::<Vec<_>>();

                    indecies.iter().for_each(|index| {
                        game.get_mut_unit(index).unwrap().add_life();
                    });
                }
            });

            ui.separator();

            ui.label("Use arrows to rotate the camera");
            ui.label("Use Enter to pause/unpause");
            ui.label("");
            ui.label("Left-click on a sphere to mark cell");
            ui.label("Right-click to unmark");
            ui.label("Choose pattern to spawn it around of each marked cell");

            ui.separator();
            ui.separator();

            if ui.add(Button::new("Log marked")).clicked() {
                use log::info;
                info!("{:?}", self.msges);
            }
        });
    }

    pub fn handle_mouse_clicks(
        &mut self,
        geometry: impl IntoIterator<Item = impl three_d::Geometry>,
        camera: &mut three_d::Camera,
        context: &three_d::Context,
        game: &mut engine::game::Game,
        (position, button): (&three_d::LogicalPoint, &three_d::MouseButton),
    ) {
        if let Some(three_d::Vector3 { x, y, z }) =
            three_d::renderer::pick(context, camera, position, geometry)
        {
            let index = engine::game::as_spherical(&(x as f64, y as f64, z as f64))
                .to_cell(engine::data::RESOLUTION);

            match button {
                three_d::MouseButton::Left => game.mark_unit(index),
                three_d::MouseButton::Right => game.unmark_unit(index),
                _ => (),
            }

            self.skip_frame = true;
            self.msges.push(game.get_raw_coords(index));
        }
    }

    pub fn handle_keyboard_event(
        &mut self,
        camera: &mut three_d::Camera,
        kind: &three_d::renderer::control::Key,
    ) {
        use three_d::renderer::control::Key;

        let speed = 0.5;

        match kind {
            Key::ArrowDown => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                -speed,
            ),
            Key::ArrowLeft => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                -speed,
                0.0,
            ),
            Key::ArrowRight => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                speed,
                0.0,
            ),
            Key::ArrowUp => camera.rotate_around(
                &three_d::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
                speed,
            ),
            Key::Enter | Key::Space => self.toogle_pause(),
            _ => (),
        }
    }


    pub fn update_game_state(&mut self, game: &mut engine::game::Game){
        if !self.pause && !self.skip_frame {
            game.next_tick();
            game.swap_buffers();
        }

        self.skip_frame = false;
    }
}

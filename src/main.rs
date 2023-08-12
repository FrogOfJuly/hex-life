use three_d::*;

fn lnlt_to_unit_sphere(ltln: &h3o::LatLng) -> (f64, f64, f64) {
    let (lt, ln) = (ltln.lat_radians(), ltln.lng_radians());
    (lt.cos() * ln.cos(), lt.cos() * ln.sin(), lt.sin())
}

fn boundary_to_ordered_vtxes(
    game: &engine::game::Game,
    index: &h3o::CellIndex,
) -> std::vec::Vec<(three_d::Vector3<f64>, three_d::Srgba)> {
    let boundary = index.boundary();
    boundary
        .iter()
        .zip(boundary.iter().cycle().skip(1))
        .zip([h3o::LatLng::from(*index)].into_iter().cycle())
        .flat_map(|((i, j), c)| {
            [
                lnlt_to_unit_sphere(i),
                lnlt_to_unit_sphere(j),
                lnlt_to_unit_sphere(&c),
            ]
            .into_iter()
        })
        .map(|(x, y, z)| Vector3 { x, y, z })
        .zip(
            {
                let unit = game
                    .present
                    .0
                    .get(&engine::game::SphericalIndex(*index))
                    .unwrap();
                let [r, g, b, a] = unit.compute_color();
                let tr = |x: f32| {
                    let f2 = x.min(1.0).max(0.0);
                    if f2 == 1.0 {
                        255
                    } else {
                        (f2 * 256.0) as u8
                    }
                };
                [Srgba {
                    r: tr(r),
                    g: tr(g),
                    b: tr(b),
                    a: tr(a),
                }]
            }
            .into_iter()
            .cycle(),
        )
        .collect::<Vec<_>>()
}

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Life".to_string(),
        max_size: None,
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

    let mut game = engine::game::Game::new();
    let resolution = h3o::Resolution::Two;

    let indecies = h3o::CellIndex::base_cells()
        .flat_map(|index| index.children(resolution))
        .collect::<Vec<_>>();

    indecies.iter().for_each(|index| {
        let mut unit = engine::unit::UnitData::new();
        unit.randomize_life(0.5);

        game.present
            .0
            .insert(engine::game::SphericalIndex(*index), unit);
        game.future
            .0
            .insert(engine::game::SphericalIndex(*index), unit);
    });

    let mut pause = true;

    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
    {
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("Controls").show(gui_context, |ui| {
                    ui.vertical_centered(|ui|{
                        ui.label("Controls");
                        if ui.add(Button::new(if pause {"Run"} else {"Pause"})).clicked(){
                            pause = !pause;
                        }

                        if ui.add(Button::new("Clear")).clicked(){
                            game.present.0.iter_mut().for_each(|(_k, v)| {
                                v.inhabited = false;
                            })
                        }

                        if ui.add(Button::new("Fill")).clicked(){
                            game.present.0.iter_mut().for_each(|(_k, v)| {
                                v.randomize_life(0.5);
                            })
                        }

                        ui.separator();

                        ui.label("Use arrows to rotate the camera");
                        ui.label("Use Enter to pause/unpause");
                })});
            },
        );


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
                .flat_map(|index| boundary_to_ordered_vtxes(&game, index))
                .unzip();

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
            )
            .write(|| gui.render());

        game.next_tick();
        if !pause{
            game.swap_buffers();
        }

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}

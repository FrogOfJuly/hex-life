use three_d::*;

#[path = "ui.rs"]
mod ui;

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

    let mut gui_state = ui::GUIState::new();

    let mut game = engine::game::Game::new().with_spawned_life();

    // Start the main render loop
    window.render_loop(move |mut frame_input| {
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| gui_state.draw_ui(gui_context, &mut game),
        );
        camera.set_viewport(frame_input.viewport);

        let (vtxes, colors) = game
            .indecies
            .iter()
            .flat_map(|x| game.cell_to_colored_face_vtxes(x))
            .map(|((x, y, z), color)| {
                (Vector3 { x, y, z }, {
                    let [r, g, b, a] = color.map(|x| {
                        let f2 = x.min(1.0).max(0.0);
                        if f2 == 1.0 {
                            255
                        } else {
                            (f2 * 256.0) as u8
                        }
                    });
                    Srgba { r, g, b, a }
                })
            })
            .unzip();

        let model = Gm::new(
            Mesh::new(
                &context,
                &CpuMesh {
                    positions: Positions::F64(vtxes),
                    colors: Some(colors),
                    ..Default::default()
                },
            ),
            ColorMaterial::default(),
        );

        gui_state.skip_frame = false;

        frame_input.events.iter().for_each(|event| match event {
            Event::MousePress {
                button, position, ..
            } => gui_state.handle_mouse_clicks(
                &model.geometry,
                &mut camera,
                &context,
                &mut game,
                (position, button),
            ),
            Event::KeyPress { kind, .. } => {
                gui_state.handle_keyboard_event(&context, &mut camera, &mut game, kind)
            }
            _ => (),
        });

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &model, &[])
            .write(|| gui.render());

        if !gui_state.pause && !gui_state.skip_frame {
            game.next_tick();
            game.swap_buffers();
        }

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
}

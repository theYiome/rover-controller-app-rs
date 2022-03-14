use std::ops::RangeInclusive;

use glium::glutin::event_loop;
use glium::{glutin, Surface};
use glutin::event::ElementState;
use glutin::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use crate::graphics::{self, InstanceAttribute};

pub fn run_rover_egui_app() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let display = {
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize {
                width: 1280 as u32,
                height: 720 as u32,
            })
            .with_title("rover-controller-app-rs");

        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);

        glium::Display::new(wb, cb, &event_loop).unwrap()
    };

    let vertex_shader_src = std::fs::read_to_string("glsl/vertex.vert").unwrap();
    let fragment_shader_src = std::fs::read_to_string("glsl/fragment.frag").unwrap();

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let mut egui_glium = egui_glium::EguiGlium::new(&display);

    // let (disk_verticies, disk_indices) = graphics::disk_mesh(16);
    let (disk_verticies, disk_indices) = graphics::square_mesh();

    let disk_vertex_buffer = glium::VertexBuffer::immutable(&display, &disk_verticies).unwrap();

    let disk_index_buffer = glium::IndexBuffer::immutable(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &disk_indices,
    )
    .unwrap();

    let instances: Vec<graphics::InstanceAttribute> = vec![
        InstanceAttribute {
            position: [0.2, 0.2],
            scale_x: 0.3,
            scale_y: 0.3,
            rotation: 0.0,
            color: [0.4, 0.4, 0.4],
        },
        InstanceAttribute {
            position: [-0.2, -0.2],
            scale_x: 0.3,
            scale_y: 0.3,
            rotation: 0.0,
            color: [0.8, 0.8, 0.8],
        },
        InstanceAttribute {
            position: [0.2, -0.2],
            scale_x: 0.4,
            scale_y: 0.4,
            rotation: 0.0,
            color: [0.1, 0.1, 0.1],
        },
    ];

    let mut max_speed: f32 = 1.0;
    let mut zoom: f32 = 1.0;

    let mut redraw_clousure = move |display: &glium::Display, egui_glium: &mut egui_glium::EguiGlium| {
        egui_glium.run(&display, |egui_ctx| {
            egui::SidePanel::left("left-side-panel").show(egui_ctx, |ui| {
                ui.heading("General settings");
                ui.separator();
                ui.label("Max speed");
                ui.add(egui::Slider::new(
                    &mut max_speed,
                    RangeInclusive::new(0.0, 2.0),
                ));
                ui.separator();
                ui.label("Zoom");
                ui.add(egui::Slider::new(
                    &mut zoom,
                    RangeInclusive::new(-2.0, 2.0),
                ));
            });

            egui::SidePanel::right("right-side-panel").show(egui_ctx, |ui| {
                ui.heading("Sensors data");
                ui.separator();
                if ui.button("Get sensor data").clicked() {
                    println!("Gettings sensor data...");
                }
                ui.separator();
                ui.label(format!("Temperature: {}°C", 20));
                ui.separator();

                let texture: &egui::TextureHandle = &ui.ctx().load_texture("my-image", egui::ColorImage::example());
                ui.label("Camera 01");
                ui.image(texture, texture.size_vec2());
                ui.label("Camera 02");
                ui.image(texture, texture.size_vec2());
            });
        });

        let mut target = display.draw();
        // draw things behind egui here
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let instance_buffer = glium::VertexBuffer::dynamic(display, &instances).unwrap();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };


        let window_size = display.gl_window().window().inner_size();

        target
            .draw(
                (&disk_vertex_buffer, instance_buffer.per_instance().unwrap()),
                &disk_index_buffer,
                &program,
                &glium::uniform! {screen_ratio: window_size.width as f32 / window_size.height as f32, zoom: zoom},
                &params,
            )
            .unwrap();

        // draw egui
        egui_glium.paint(&display, &mut target);

        // draw things on top of egui here
        target.finish().unwrap();
    };

    let main_loop = move |event: Event<()>,
                          _: &event_loop::EventLoopWindowTarget<()>,
                          control_flow: &mut ControlFlow| {
        match event {
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw_clousure(&display, &mut egui_glium),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw_clousure(&display, &mut egui_glium),

            glutin::event::Event::WindowEvent { event, .. } => {
                egui_glium.on_event(&event);
                match event {
                    WindowEvent::CloseRequested {} => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => {
                        if input.virtual_keycode == Some(VirtualKeyCode::F1)
                            && input.state == ElementState::Pressed
                        {
                            println!("F1");
                        }
                    }
                    _ => (),
                }
                display.gl_window().window().request_redraw();
            }
            glutin::event::Event::RedrawRequested { .. } => {
                redraw_clousure(&display, &mut egui_glium);
                display.gl_window().window().request_redraw();
            }
            Event::MainEventsCleared => {
                redraw_clousure(&display, &mut egui_glium);
                display.gl_window().window().request_redraw();
            }
            _ => (),
        }
    };

    // do execute main loop clousure
    event_loop.run(main_loop);
}
